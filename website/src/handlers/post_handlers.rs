use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use serde_json::json;
use std::sync::Arc;

use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    pub title: String,
}

pub async fn create_post_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let result: Result<Vec<Post>, _> = db
        .insert("posts")
        .content(payload)
        .await;

    match result {
        Ok(mut posts) => {
            let post = posts.pop().unwrap();
            (StatusCode::CREATED, Json(post)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_posts_handler(
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let result: Result<Vec<Post>, _> = db.select("posts").await;

    match result {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
