use crate::api::Result;
use serde::{Deserialize, Serialize};
use theseus::prelude::*;

// Identical to theseus::settings::Settings except for the custom_java_args field
// This allows us to split the custom_java_args string into a Vec<String> here and join it back into a string in the backend
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrontendSettings {
    pub theme: Theme,
    pub memory: MemorySettings,
    pub game_resolution: WindowSize,
    pub custom_java_args: String,
    pub custom_env_args: String,
    pub java_globals: JavaGlobals,
    pub default_user: Option<uuid::Uuid>,
    pub hooks: Hooks,
    pub max_concurrent_downloads: usize,
    pub max_concurrent_writes: usize,
    pub version: u32,
    pub collapsed_navigation: bool,
}

// Get full settings
// invoke('settings_get')
#[tauri::command]
pub async fn settings_get() -> Result<Settings> {
    let res = settings::get().await?;
    Ok(res)
}

// Set full settings
// invoke('settings_set', settings)
#[tauri::command]
pub async fn settings_set(settings: Settings) -> Result<()> {
    settings::set(settings).await?;
    Ok(())
}
