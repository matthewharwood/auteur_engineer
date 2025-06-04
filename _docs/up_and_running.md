
# Up and Running


## Make a Template using folder_name/index.html
e.g. templates/counter/

## Make a Handler
e.g. src/handlers/counter_handler.rs

```rust
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use serde::Serialize;
use tera::{Context, Tera};

#[derive(Serialize)]
pub struct IndexPageData<'a> {
    pub title: &'a str,
}
pub async fn page_handler( State(tera): State<Arc<Tera>>) -> impl IntoResponse {
    let mut context = Context::new();
    let page_data = IndexPageData {
        title: "Auteur.Engineer (from index_handler)",
    };
    context.insert("page", &page_data);
    match tera.render("index.html", &context) {
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
```

## Create route in main.rs
```rust
#[tokio::main]
async fn main() {
    let app = Router::new().route("/counter", get(handlers::counter_handler::page_handler));
}
```
