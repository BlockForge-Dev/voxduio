use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: Option<String>,
    pub data: Option<T>,
}

pub fn json_response<T: Serialize>(status: StatusCode, payload: ApiResponse<T>) -> Response {
    (status, Json(payload)).into_response()
}
