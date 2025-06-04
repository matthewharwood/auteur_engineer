use std::sync::Arc;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Form, Json};
use axum::response::{Html, IntoResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;
use tera::{Context};
use plat_schema_macros::PlatSchema;
use crate::AppState;
use crate::handlers::post_handlers::{Field, FormType, Post};

#[derive(Serialize, Deserialize, PlatSchema, Debug)]
pub struct Counter {
    pub id: Option<Thing>,
    pub count: i32,
}
#[derive(Deserialize, Serialize,Debug)]
pub struct CreateCounter {
    pub count: i32,
}
#[derive(Deserialize)]
pub struct CounterAction {
    pub action: String,
}
pub async fn page_handler( State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let tera = &app_state.templates;
    let db   = &app_state.db;

    // 1) Fetch all posts
    let counter_res: Result<Vec<Counter>, _> = db.select("Counter").await;
    match counter_res {
        Ok(posts) => {
            // 2) Insert into Tera context
            let mut context = Context::new();
            context.insert("data", &posts);

            // 3) Render the template
            match tera.render("counter/index.html", &context) {
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

pub async fn create_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Form(form): Form<CounterAction>,
) -> impl IntoResponse {
    match form.action.as_str() {
        "inc" => println!("incrementing counter"),
        "dec" => println!("decrement counter"),
        _ => println!("nada counter")
    }
    // let obj = Counter {
    //     id: None,
    //     count: payload.count.clone(),
    // };
    // let db = &app_state.db;
    // let result: Result<Vec<Counter>, _> = db.insert("counter").content(obj).await;
    //
    // match result {
    //     Ok(mut counters) => {
    //         let obj = counters.pop().unwrap();
    //         (StatusCode::CREATED, Json(obj)).into_response()
    //     }
    //     Err(e) => (
    //         StatusCode::INTERNAL_SERVER_ERROR,
    //         Json(json!({"error": e.to_string()})),
    //     )
    //         .into_response(),
    // }
}