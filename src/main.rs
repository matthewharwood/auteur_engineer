mod handlers;

use axum::{
    response::IntoResponse,
    routing::get,
    Router,
};

use std::net::SocketAddr;
use std::sync::Arc;
use tera::{Context, Tera};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;




#[tokio::main]
async fn main() {
    let tera_instance = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("FATAL: Parsing error(s) on template initialization: {}", e);
            ::std::process::exit(1);
        }
    };
    // Wrap the Tera instance in an Arc to allow shared ownership across threads
    let shared_tera = Arc::new(tera_instance);
    let static_files_service = ServeDir::new("public")
        .append_index_html_on_directories(false);
    let app = Router::new()
        .route("/", get(handlers::index_handler::serve_index_page_handler))
        .route("/api/hello", get(handlers::api_handlers::hello_json_api_handler))
        .nest_service("/public", static_files_service)
        .with_state(shared_tera);

    // TODO(harwood) refactor this to make work when hosted in cloud
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
