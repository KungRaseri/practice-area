use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    status: String,
    message: String,
}
