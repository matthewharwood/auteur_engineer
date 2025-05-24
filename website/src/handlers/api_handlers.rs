// src/handlers/api_handlers.rs
use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse { // Made pub
    pub message: String, // Made fields pub for potential external use if struct is pub
    pub status_code: u16,
    pub data: Option<String>,
}

pub async fn hello_json_api_handler() -> impl IntoResponse { // Renamed for clarity, made pub
    let response_data = ApiResponse {
        message: "hello_JSON from api_handler".to_string(),
        status_code: 200,
        data: Some("this is some example data".to_string()),
    };
    Json(response_data)
}