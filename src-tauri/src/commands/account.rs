mod structs;
use structs::account::Account;
mod models;
use models::account::*;

pub async fn create_account (state: tauri::State<_, DbState>, data: Account) -> Result<(), String>{
	let db = &state.connection;

	let user = account::ActiveModel {
		name: Set(data.email.to_owned()),
		password: Set(data.password.to_owned()),
		..Default::default()
	};

	match user.insert(&db).await {
		Ok(user) => Response {
			ok: true,
			message: "Created Account Successfully",
			data: Some(user)
		},
		Err(e) => Response {
			ok: false,
			message: "Account Creation Failed",
			data: None
		}
	}
}