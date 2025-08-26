use sea_orm::DatabaseConnection;

mod services;
use services::database::init_db;

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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
