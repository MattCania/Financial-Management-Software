use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub struct Account {
	pub(crate) email: String,
	pub password: String
}