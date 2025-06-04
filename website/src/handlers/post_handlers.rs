use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use axum::extract::Path;
use axum::response::Html;
use tera::Context;
use crate::AppState;
use crate::schema::{self, Post, Field, FormType};

#[derive(Deserialize, Serialize,Debug)]
pub struct CreatePost {
    pub title: String,
}


pub async fn serve_admin_page_index_handler(
    State(app_state): State<Arc<AppState>>
) -> impl IntoResponse {
    let tera = &app_state.templates;
    let db   = &app_state.db;

    // 1) Fetch all posts
    let posts_res: Result<Vec<Post>, _> = db.select("posts").await;
    

    match posts_res {
        Ok(posts) => {
            // 2) Insert into Tera context
            let mut context = Context::new();
            context.insert("posts", &posts);

            // 3) Render the template
            match tera.render("admin/posts/index.html", &context) {
                Ok(html) => Html(html).into_response(),
                Err(err) => {
                    eprintln!("Template error: {}", err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Template error: {}", err),
                    )
                        .into_response()
                }
            }
        },
        Err(e) => {
            // 4) Handle DB error
            eprintln!("DB error fetching posts: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error loading posts: {}", e),
            )
                .into_response()
        }
    }
}


pub async fn serve_admin_page_id_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tera = &app_state.templates;
    let db = &app_state.db;
    let posts_data: Option<Post> = match db.select(("posts", id.clone())).await {
        Ok(post) => post,
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    let Some(post) = posts_data else {
        return (StatusCode::NOT_FOUND, "Post not found").into_response();
    };

    let page_schema = if post.blocks.is_empty() {
        schema::default_page_schema()
    } else {
        post.blocks.clone()
    };

    let mut context = Context::new();
    context.insert("post", &post);
    context.insert("page_schema", &page_schema);

    match tera.render("admin/posts/[id].html", &context) {
        Ok(html) => Html(html).into_response(),
        Err(err) => {
            eprintln!("Template rendering error: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template: {}", err),
            )
                .into_response()
        }
    }
}

pub async fn create_post_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
    let title_field = Field {
        label:      payload.title.clone(),
        hint:       "".into(),              // or something sensible
        form_type:  FormType::InputText,    // choose the right variant
    };

    let new_post = Post {
        id: None,
        title:  title_field,
        blocks: vec![],
    };
    let db = &app_state.db;
    let result: Result<Vec<Post>, _> = db.insert("posts").content(new_post).await;

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

    match result {
        Ok(posts) => Json(posts).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn update_post_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<Post>,
) -> impl IntoResponse {
    let db = &app_state.db;

    let result: Result<Option<Post>, _> = db
        .update(("posts", id))
        .content(payload)
        .await;

    match result {
        Ok(Some(post)) => (StatusCode::OK, Json(post)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Post not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
