use std::sync::{Arc, Mutex};
use tauri::command;
use crate::AppState;

#[command]
pub fn get_contacts(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<String, String> {
    return Ok("aaa".parse().unwrap());
}