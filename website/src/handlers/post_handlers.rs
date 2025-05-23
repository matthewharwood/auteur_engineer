use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use plat_schema::Schema;
use plat_schema_macros::PlatSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use axum::response::Html;
use surrealdb::sql::Thing;
use tera::Context;
use crate::AppState;

#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Post {
    pub id: Option<Thing>,
    pub title: Field,
    pub blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, PlatSchema)]
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

#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Header {
    pub content: Field,
}

#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Footer {
    pub copyright: Field,
}

#[derive(Deserialize, Serialize)]
pub enum FormType {
    InputArea,
    InputText,
    InputDate,
}
#[derive(Serialize, Deserialize, PlatSchema)]
pub struct Field {
    pub label: String,
    pub hint: String,
    pub form_type: FormType,
}

#[derive(Deserialize, Serialize)]
pub struct CreatePost {
    pub title: String,
}
#[derive(Deserialize, Serialize)]
pub struct PageData<'a> {
    form_name: &'a str,
    title: Field,
    blocks: Vec<Block>,
}

pub async fn serve_admin_page_id_handler(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let tera = &app_state.templates;
    let mut context = Context::new();
    let page_data = PageData {
        form_name: Post::name(),
        title: Field {
            label: "Page B".to_string(),
            hint: "Enter the title of your post".to_string(),
            form_type: FormType::InputArea,
        },
        blocks: vec![Block::Header(Header {
            content: Field {
                label: "Content".to_string(),
                hint: "Enter the content of your post".to_string(),
                form_type: FormType::InputText,
            },
        })],
    };
    context.insert("page", &page_data);
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
