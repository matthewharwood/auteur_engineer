use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use plat_schema::Schema;
use plat_schema_macros::PlatSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use surrealdb::sql::Thing;

use crate::AppState;

#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: String,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, PlatSchema)]
pub enum Block {
    Header(Header),
    Footer(Footer),
}

#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Header {
    pub text: String,
}
#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Footer {
    pub copyright: String,
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
    let result: Result<Vec<Post>, _> = db.insert("posts").content(payload).await;

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

pub async fn get_posts_handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let db = &app_state.db;
    let result: Result<Vec<Post>, _> = db.select("posts").await;
    println!("{}", Post::name());
    match result {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
