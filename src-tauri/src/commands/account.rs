use sea_orm::{ActiveModelTrait, Set, EntityTrait};
use tauri::State;

use crate::models::account;
use crate::types::account::Account;
use crate::types::response::RequestResponse;
use crate::{models, DbState};

#[tauri::command]
pub async fn create_account(
    state: State<'_, DbState>,
    data: Account
) -> Result<RequestResponse<account::Model>, String> {
    let db = &state.connection;

    let user = account::ActiveModel {
        email: Set(data.email.to_owned()),
        password: Set(data.password.to_owned()),
        ..Default::default()
    };

    match user.insert(db).await {
        Ok(user) => Ok(RequestResponse {
            ok: true,
            message: "Created Account Successfully".to_string(),
            data: Some(user),
        }),
        Err(e) => Ok(RequestResponse {
            ok: false,
            message: format!("Account Creation Failed: {}", e),
            data: None,
        }),
    }
}

#[tauri::command]
pub async fn get_accounts(
    state: State<'_, DbState>
) -> Result<RequestResponse<Vec<account::Model>>, String> {
    let db = &state.connection;

    match account::Entity::find().all(db).await {
        Ok(accs) => Ok(RequestResponse {
            ok: true,
            message: "Accounts retrieved successfully".to_string(),
            data: Some(accs),
        }),
        Err(e) => Err(format!("Database error: {}", e)),
    }
}