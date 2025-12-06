use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "https://api-dev.pondokinformatika.id/api";

#[derive(Deserialize)]
struct ApiResponse<T> {
    success: bool,
    message: Option<String>,
    data: Option<T>,
}

#[derive(Deserialize)]
pub struct ActivateData {
    pub santri_id: i64,
    pub nama: String,
}

pub struct ActivateResult {
    pub success: bool,
    pub santri_id: i64,
    pub santri_name: String,
    pub message: String,
}

pub struct StartTrackingResult {
    pub success: bool,
    pub tracking_id: i64,
}

pub async fn activate_agent(token: &str) -> Result<ActivateResult, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    let response = client
        .post(&format!("{}/tracking/agent/activate", API_BASE))
        .json(&serde_json::json!({
            "token": token,
            "device_id": get_device_id(),
            "device_name": get_device_name(),
            "os": std::env::consts::OS,
            "os_version": "",
            "agent_version": "1.0.0"
        }))
        .send()
        .await?;
    
    let result: ApiResponse<ActivateData> = response.json().await?;
    
    if result.success {
        if let Some(data) = result.data {
            return Ok(ActivateResult {
                success: true,
                santri_id: data.santri_id,
                santri_name: data.nama,
                message: "Aktivasi berhasil".to_string(),
            });
        }
    }
    
    Ok(ActivateResult {
        success: false,
        santri_id: 0,
        santri_name: String::new(),
        message: result.message.unwrap_or_else(|| "Token tidak valid".to_string()),
    })
}

pub async fn heartbeat(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    client
        .post(&format!("{}/tracking/agent/heartbeat", API_BASE))
        .json(&serde_json::json!({ "token": token }))
        .send()
        .await?;
    
    Ok(())
}

pub async fn start_tracking(token: &str, rencana: &str) -> Result<StartTrackingResult, Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // We need to call the santri start endpoint, but the agent uses token-based auth
    // For now, we'll use a workaround - the actual implementation should use Sanctum token
    // This is a placeholder - in production, handle auth properly
    
    Ok(StartTrackingResult {
        success: true,
        tracking_id: 1, // Placeholder
    })
}

pub async fn end_tracking(token: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder - implement actual API call
    Ok(())
}

pub async fn submit_report(token: &str, hasil: &str, kendala: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    // Placeholder - implement actual API call
    Ok(())
}

#[derive(Serialize)]
pub struct Activity {
    pub tipe: String,
    pub nama: String,
    pub window_title: String,
    pub url: Option<String>,
    pub durasi: i64,
    pub recorded_at: String,
}

pub async fn sync_activities(token: &str, tracking_id: i64, activities: Vec<Activity>) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    client
        .post(&format!("{}/tracking/agent/sync", API_BASE))
        .json(&serde_json::json!({
            "token": token,
            "tracking_id": tracking_id,
            "activities": activities
        }))
        .send()
        .await?;
    
    Ok(())
}

fn get_device_id() -> String {
    // Get unique device identifier
    // This is a simplified implementation
    format!("device-{}", uuid_simple())
}

fn get_device_name() -> String {
    hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "Unknown".to_string())
}

fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    format!("{:x}", duration.as_nanos())
}
