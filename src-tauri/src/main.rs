#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tracker;
mod storage;
mod api;
mod tray;

use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct TrackingState {
    pub status: String, // standby, active, paused, idle
    pub tracking_id: Option<i64>,
    pub duration: i64,
    pub productive_duration: i64,
    pub idle_duration: i64,
    pub current_app: String,
    pub current_window: String,
}

#[derive(Default, Clone, Serialize)]
pub struct AppUsage {
    pub name: String,
    pub duration: i64,
    pub category: String, // productive, neutral, unproductive
}

#[derive(Default)]
pub struct AppState {
    pub is_activated: bool,
    pub santri_id: Option<i64>,
    pub santri_name: String,
    pub token: String,
    pub tracking: TrackingState,
    pub app_usage: Vec<AppUsage>,
}

type SharedState = Arc<Mutex<AppState>>;

#[derive(Serialize)]
struct ActivationCheckResult {
    is_activated: bool,
    santri_id: Option<i64>,
    santri_name: String,
    token: String,
}

#[derive(Serialize)]
struct ActivationResult {
    success: bool,
    santri_id: i64,
    santri_name: String,
    message: String,
}

#[derive(Serialize)]
struct StartResult {
    success: bool,
    tracking_id: i64,
}

#[tauri::command]
async fn check_activation(state: State<'_, SharedState>) -> Result<ActivationCheckResult, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    
    // Try to load from local storage
    if let Ok(stored) = storage::load_activation() {
        return Ok(ActivationCheckResult {
            is_activated: true,
            santri_id: Some(stored.santri_id),
            santri_name: stored.santri_name,
            token: stored.token,
        });
    }
    
    Ok(ActivationCheckResult {
        is_activated: app_state.is_activated,
        santri_id: app_state.santri_id,
        santri_name: app_state.santri_name.clone(),
        token: app_state.token.clone(),
    })
}

#[tauri::command]
async fn activate_token(token: String, state: State<'_, SharedState>) -> Result<ActivationResult, String> {
    // Call API to activate
    match api::activate_agent(&token).await {
        Ok(result) => {
            if result.success {
                // Save to local storage
                storage::save_activation(&storage::ActivationData {
                    santri_id: result.santri_id,
                    santri_name: result.santri_name.clone(),
                    token: token.clone(),
                })?;
                
                // Update state
                let mut app_state = state.lock().map_err(|e| e.to_string())?;
                app_state.is_activated = true;
                app_state.santri_id = Some(result.santri_id);
                app_state.santri_name = result.santri_name.clone();
                app_state.token = token;
                
                Ok(ActivationResult {
                    success: true,
                    santri_id: result.santri_id,
                    santri_name: result.santri_name,
                    message: "Aktivasi berhasil".to_string(),
                })
            } else {
                Ok(ActivationResult {
                    success: false,
                    santri_id: 0,
                    santri_name: String::new(),
                    message: result.message,
                })
            }
        }
        Err(e) => Ok(ActivationResult {
            success: false,
            santri_id: 0,
            santri_name: String::new(),
            message: e.to_string(),
        }),
    }
}

#[tauri::command]
async fn get_tracking_state(state: State<'_, SharedState>) -> Result<TrackingState, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    Ok(app_state.tracking.clone())
}

#[tauri::command]
async fn get_app_usage(state: State<'_, SharedState>) -> Result<Vec<AppUsage>, String> {
    let app_state = state.lock().map_err(|e| e.to_string())?;
    Ok(app_state.app_usage.clone())
}

#[tauri::command]
async fn start_tracking(
    rencana_belajar: String,
    token: String,
    state: State<'_, SharedState>,
) -> Result<StartResult, String> {
    match api::start_tracking(&token, &rencana_belajar).await {
        Ok(result) => {
            if result.success {
                let mut app_state = state.lock().map_err(|e| e.to_string())?;
                app_state.tracking.status = "active".to_string();
                app_state.tracking.tracking_id = Some(result.tracking_id);
                app_state.tracking.duration = 0;
                app_state.tracking.productive_duration = 0;
                app_state.tracking.idle_duration = 0;
                
                Ok(StartResult {
                    success: true,
                    tracking_id: result.tracking_id,
                })
            } else {
                Ok(StartResult {
                    success: false,
                    tracking_id: 0,
                })
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn pause_tracking(state: State<'_, SharedState>) -> Result<(), String> {
    let mut app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.tracking.status = "paused".to_string();
    Ok(())
}

#[tauri::command]
async fn resume_tracking(state: State<'_, SharedState>) -> Result<(), String> {
    let mut app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.tracking.status = "active".to_string();
    Ok(())
}

#[tauri::command]
async fn end_tracking(state: State<'_, SharedState>) -> Result<(), String> {
    let token = {
        let app_state = state.lock().map_err(|e| e.to_string())?;
        app_state.token.clone()
    };
    
    api::end_tracking(&token).await.map_err(|e| e.to_string())?;
    
    let mut app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.tracking.status = "standby".to_string();
    Ok(())
}

#[tauri::command]
async fn submit_report(
    hasil_belajar: String,
    kendala: Option<String>,
    token: String,
    state: State<'_, SharedState>,
) -> Result<(), String> {
    api::submit_report(&token, &hasil_belajar, kendala.as_deref()).await.map_err(|e| e.to_string())?;
    
    let mut app_state = state.lock().map_err(|e| e.to_string())?;
    app_state.tracking = TrackingState::default();
    app_state.tracking.status = "standby".to_string();
    app_state.app_usage.clear();
    
    Ok(())
}

fn main() {
    let shared_state: SharedState = Arc::new(Mutex::new(AppState::default()));
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(shared_state.clone())
        .setup(move |app| {
            // Setup system tray
            tray::setup_tray(app)?;
            
            // Start background tracker
            let state_clone = shared_state.clone();
            std::thread::spawn(move || {
                tracker::start_tracking_loop(state_clone);
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_activation,
            activate_token,
            get_tracking_state,
            get_app_usage,
            start_tracking,
            pause_tracking,
            resume_tracking,
            end_tracking,
            submit_report,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
