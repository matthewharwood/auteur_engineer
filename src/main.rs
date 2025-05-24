mod handlers;

use axum::{
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use std::net::SocketAddr;
use std::sync::Arc;
use tera::{Context, Tera};
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;


pub struct AppState {
    pub templates: Arc<Tera>,
    pub db: Arc<Surreal<Ws>>,
}



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

    let db = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("test").use_db("test").await.unwrap();
    let shared_db = Arc::new(db);

    let app_state = Arc::new(AppState {
        templates: shared_tera.clone(),
        db: shared_db,
    });

    let static_files_service = ServeDir::new("public")
        .append_index_html_on_directories(false);
    let app = Router::new()
        .route("/", get(handlers::index_handler::serve_index_page_handler))
        .route("/mario", get(handlers::mario_index_handler::serve_index_page_handler))
        .route("/api/hello", get(handlers::api_handlers::hello_json_api_handler))
        .route(
            "/api/posts",
            post(handlers::post_handlers::create_post_handler)
                .get(handlers::post_handlers::get_posts_handler),
        )
        .nest_service("/public", static_files_service)
        .with_state(app_state);

    // TODO(harwood) refactor this to make work when hosted in cloud
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
