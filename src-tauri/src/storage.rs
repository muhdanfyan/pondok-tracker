use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct ActivationData {
    pub santri_id: i64,
    pub santri_name: String,
    pub token: String,
}

fn get_data_dir() -> Result<PathBuf, String> {
    let dir = dirs::data_dir()
        .ok_or("Cannot find data directory")?
        .join("PondokTracker");
    
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    }
    
    Ok(dir)
}

pub fn save_activation(data: &ActivationData) -> Result<(), String> {
    let path = get_data_dir()?.join("activation.json");
    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn load_activation() -> Result<ActivationData, String> {
    let path = get_data_dir()?.join("activation.json");
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_json::from_str(&content).map_err(|e| e.to_string())
}

pub fn clear_activation() -> Result<(), String> {
    let path = get_data_dir()?.join("activation.json");
    if path.exists() {
        fs::remove_file(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
