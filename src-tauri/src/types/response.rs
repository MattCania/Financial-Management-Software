use serde::Serialize;

#[derive(Serialize)]
pub struct RequestResponse<T> {
	pub ok: bool,
	pub message: String,
	pub data: Option<T>
}