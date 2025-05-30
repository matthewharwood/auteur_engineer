use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use plat_schema::Schema;
use plat_schema_macros::PlatSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use axum::extract::Path;
use axum::response::Html;
use surrealdb::sql::Thing;
use tera::Context;
use crate::AppState;

#[derive(Serialize, Deserialize, PlatSchema, Debug)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: Field,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, PlatSchema, Debug)]
pub enum Block {
    Header(Header),
    Footer(Footer),
    // Ref(BlockRef),
}

// #[derive(Serialize, Deserialize, PlatSchema)]
// pub struct BlockRef {
//     pub post_id:     Thing,
//     pub block_index: usize,
// }

#[derive(Serialize, Deserialize, PlatSchema, Debug)]
pub struct Header {
    pub content: Field,
}

#[derive(Serialize, Deserialize, PlatSchema, Debug)]
pub struct Footer {
    pub copyright: Field,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum FormType {
    InputArea,
    InputText,
    InputDate,
}
#[derive(Serialize, Deserialize, PlatSchema, Debug)]
pub struct Field {
    pub label: String,
    pub hint: String,
    pub form_type: FormType,
}

#[derive(Deserialize, Serialize,Debug)]
pub struct CreatePost {
    pub title: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PageData {
    title: Field,
    blocks: Vec<Block>,
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


pub async fn serve_admin_page_id_handler(State(app_state): State<Arc<AppState>>,  Path(id): Path<String>,) -> impl IntoResponse {

    let tera = &app_state.templates;
    let db   = &app_state.db;
    let posts_data: Option<Post> = match db.select(("posts", id)).await {
        Ok(post) => post,
        Err(e) => {
            eprintln!("Database error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };
    
    let mut context = Context::new();
    context.insert("post", &posts_data);
    context.insert("schemas_json", &app_state.block_schemas_json);
    println!("{:?}", posts_data);
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

#[derive(Deserialize)]
#[serde(tag = "block_type", content = "block_data")]
pub enum UpsertBlockPayload {
    Header(Header),
    Footer(Footer),
}

pub async fn update_post_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(payload): Json<UpsertBlockPayload>,
) -> impl IntoResponse {
    let db = &app_state.db;

    let mut post: Option<Post> = match db.select(("posts", id.clone())).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response();
        }
    };

    let Some(mut post) = post else {
        return (StatusCode::NOT_FOUND, "post not found").into_response();
    };

    let block = match payload {
        UpsertBlockPayload::Header(h) => Block::Header(h),
        UpsertBlockPayload::Footer(f) => Block::Footer(f),
    };
    post.blocks.push(block);

    let result: Result<Option<Post>, _> = db
        .update(("posts", id))
        .merge(json!({"blocks": post.blocks}))
        .await;

    match result {
        Ok(Some(updated)) => Json(updated).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "post not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
