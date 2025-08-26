use std::path::PathBuf;
use sea_orm::{Database, DatabaseConnection};

pub fn get_db_path() -> PathBuf {
    let mut dir = dirs::data_local_dir().expect("failed to get local data dir");
    dir.push("FMSApplication");                 
    std::fs::create_dir_all(&dir).expect("failed to create app data dir");
    dir.push("fms.sqlite");         
    dir
}

pub async fn init_db() -> DatabaseConnection {
    let db_path = get_db_path();
    let db_url = format!("sqlite:{}", db_path.to_string_lossy());
    Database::connect(&db_url)
        .await
        .expect("failed to connect to sqlite")
}
