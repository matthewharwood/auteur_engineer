
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Serialize; // Needed for IndexPageData
use std::sync::Arc;
use tera::{Context, Tera};

// Temporary location for IndexPageData
#[derive(Serialize)]
struct IndexPageData<'a> {
    pub title: &'a str,
    pub heading: &'a str,
    pub message: &'a str,
    pub show_extra_info: bool,
}

pub async fn serve_index_page_handler( State(tera): State<Arc<Tera>>) -> impl IntoResponse {
    let mut context = Context::new();
    let page_data = IndexPageData {
        title: "Mario Page",
        heading: "Welcome to Auteur.Engineer",
        message: "This is a message for Autuer from the index_handler",
        show_extra_info: true, // Changed to test
    };
    context.insert("page", &page_data);

    match tera.render("mario/index.html", &context) {
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