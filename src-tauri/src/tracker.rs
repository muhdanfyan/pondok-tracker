use crate::{AppUsage, SharedState, TrackingState};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;

// Productive apps list
const PRODUCTIVE_APPS: &[&str] = &[
    "code", "vscode", "visual studio code",
    "sublime", "atom", "vim", "neovim",
    "phpstorm", "webstorm", "pycharm", "intellij",
    "android studio", "xcode",
    "figma", "sketch", "photoshop", "illustrator",
    "terminal", "iterm", "cmd", "powershell",
    "mysql workbench", "dbeaver", "datagrip",
    "notion", "obsidian", "typora",
];

// Unproductive apps list
const UNPRODUCTIVE_APPS: &[&str] = &[
    "steam", "discord", "spotify", "netflix",
    "game", "gaming",
];

pub fn start_tracking_loop(state: SharedState) {
    let mut last_sync = Instant::now();
    let mut app_durations: HashMap<String, i64> = HashMap::new();
    let mut idle_start: Option<Instant> = None;
    
    loop {
        thread::sleep(Duration::from_secs(1));
        
        let mut app_state = match state.lock() {
            Ok(s) => s,
            Err(_) => continue,
        };
        
        if app_state.tracking.status != "active" {
            continue;
        }
        
        // Get active window
        let (app_name, window_title) = get_active_window();
        
        // Check idle
        let idle_time = get_idle_time();
        if idle_time > 300 { // 5 minutes
            if idle_start.is_none() {
                idle_start = Some(Instant::now());
            }
            app_state.tracking.status = "idle".to_string();
        } else {
            if idle_start.is_some() {
                let idle_duration = idle_start.unwrap().elapsed().as_secs() as i64;
                app_state.tracking.idle_duration += idle_duration;
                idle_start = None;
            }
            app_state.tracking.status = "active".to_string();
        }
        
        // Update current app
        app_state.tracking.current_app = app_name.clone();
        app_state.tracking.current_window = window_title;
        
        // Update duration
        app_state.tracking.duration += 1;
        
        // Track app usage
        let category = categorize_app(&app_name);
        if category == "productive" {
            app_state.tracking.productive_duration += 1;
        }
        
        *app_durations.entry(app_name.clone()).or_insert(0) += 1;
        
        // Update app usage summary
        app_state.app_usage = app_durations
            .iter()
            .map(|(name, &duration)| AppUsage {
                name: name.clone(),
                duration,
                category: categorize_app(name),
            })
            .collect();
        
        // Sort by duration descending
        app_state.app_usage.sort_by(|a, b| b.duration.cmp(&a.duration));
        
        // Sync to server every 5 minutes
        if last_sync.elapsed() > Duration::from_secs(300) {
            // TODO: Call sync API
            last_sync = Instant::now();
        }
    }
}

fn categorize_app(app_name: &str) -> String {
    let lower = app_name.to_lowercase();
    
    for &app in PRODUCTIVE_APPS {
        if lower.contains(app) {
            return "productive".to_string();
        }
    }
    
    for &app in UNPRODUCTIVE_APPS {
        if lower.contains(app) {
            return "unproductive".to_string();
        }
    }
    
    "neutral".to_string()
}

// Platform-specific implementations

#[cfg(target_os = "windows")]
fn get_active_window() -> (String, String) {
    // Windows implementation using windows crate
    // This is a placeholder - actual implementation would use Windows API
    ("Unknown".to_string(), "Unknown".to_string())
}

#[cfg(target_os = "windows")]
fn get_idle_time() -> u64 {
    // Windows implementation
    0
}

#[cfg(target_os = "macos")]
fn get_active_window() -> (String, String) {
    // macOS implementation using cocoa/objc
    ("Unknown".to_string(), "Unknown".to_string())
}

#[cfg(target_os = "macos")]
fn get_idle_time() -> u64 {
    0
}

#[cfg(target_os = "linux")]
fn get_active_window() -> (String, String) {
    // Linux implementation using X11
    // Try to get active window using xdotool or similar
    use std::process::Command;
    
    let window_id = Command::new("xdotool")
        .args(["getactivewindow"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    
    if window_id.is_empty() {
        return ("Unknown".to_string(), "Unknown".to_string());
    }
    
    let window_name = Command::new("xdotool")
        .args(["getwindowname", &window_id])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    
    let window_pid = Command::new("xdotool")
        .args(["getwindowpid", &window_id])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string();
    
    let app_name = if !window_pid.is_empty() {
        Command::new("ps")
            .args(["-p", &window_pid, "-o", "comm="])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .unwrap_or_default()
            .trim()
            .to_string()
    } else {
        "Unknown".to_string()
    };
    
    (app_name, window_name)
}

#[cfg(target_os = "linux")]
fn get_idle_time() -> u64 {
    use std::process::Command;
    
    Command::new("xprintidle")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
        .map(|ms| ms / 1000)
        .unwrap_or(0)
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_active_window() -> (String, String) {
    ("Unknown".to_string(), "Unknown".to_string())
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_idle_time() -> u64 {
    0
}
