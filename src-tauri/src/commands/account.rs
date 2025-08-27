use sea_orm::{ActiveModelTrait, Set};

use crate::types::account::Account;
use crate::{models, DbState};
use crate::types::response::RequestResponse;

#[tauri::command]
pub async fn create_account(
    state: tauri::State<'_, DbState>,
    data: Account
) -> Result<RequestResponse<models::account::Model>, String> {
    let db = &state.connection;

    let user = models::account::ActiveModel {
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