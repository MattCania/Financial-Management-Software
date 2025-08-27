pub mod commands;
pub mod models;
pub mod services;
pub mod types;

use sea_orm::DatabaseConnection;

use services::database::init_db;
use commands::account::{create_account, get_accounts};

pub struct DbState {
    pub connection: DatabaseConnection
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    let db: DatabaseConnection = init_db().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(DbState{ connection: db })
        .invoke_handler(tauri::generate_handler![greet, create_account, get_accounts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
