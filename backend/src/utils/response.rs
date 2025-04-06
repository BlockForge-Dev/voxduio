use axum::{
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub  struct ApiResponse<T>{
    pub status: String,
    pub message: Optin<String>,
    pub data: Option<T>,
}

pub fn json_response<T: Serialize>(
    status: StatusCode,
    body:ApiResponse<T>,
) -> impl IntoResponse{
    (status, Json(body))
}