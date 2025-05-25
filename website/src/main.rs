mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tera::Tera;
use surrealdb::engine::remote::ws::{Client as WsClient, Ws};
use surrealdb::{opt::auth::Root, Surreal};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

pub struct AppState {
    pub templates: Arc<Tera>,
    pub db: Arc<Surreal<WsClient>>,
}

#[tokio::main]
async fn main() {
    let tera_instance = match Tera::new("website/templates/**/*.html") {
        Ok(t) => {
            println!("Tera templates loaded successfully.");
            t
        }
        Err(e) => {
            eprintln!("FATAL: Parsing error(s) on template initialization: {}", e);
            ::std::process::exit(1);
        }
    };
    let shared_tera = Arc::new(tera_instance);

    println!("Attempting to connect to SurrealDB at ws://127.0.0.1:8000...");
    // Use Ws as the generic type for Surreal::new()
    let db = match Surreal::new::<Ws>("127.0.0.1:8000").await {
        Ok(db_instance) => {
            println!("Successfully initiated SurrealDB connection.");
            db_instance
        }
        Err(e) => {
            eprintln!("FATAL: Could not connect to SurrealDB: {:?}", e);
            ::std::process::exit(1);
        }
    };

    println!("Attempting to sign in to SurrealDB...");
    if let Err(e) = db
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        eprintln!("FATAL: Could not sign in to SurrealDB: {:?}", e);
        ::std::process::exit(1);
    }
    println!("Successfully signed in to SurrealDB.");

    println!("Attempting to use namespace 'test' and database 'test'...");
    if let Err(e) = db.use_ns("test").use_db("test").await {
        eprintln!(
            "FATAL: Could not use namespace/database in SurrealDB: {:?}",
            e
        );
        ::std::process::exit(1);
    }
    println!("Successfully set SurrealDB namespace and database.");

    let shared_db = Arc::new(db);

    let app_state = Arc::new(AppState {
        templates: shared_tera.clone(),
        db: shared_db,
    });
    println!("AppState created successfully.");
    let public_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("public");
    println!("Public Dir: {:?}", public_dir);
    let static_files_service = ServeDir::new(public_dir).append_index_html_on_directories(false);

    let app = Router::new()
        .route(
            "/admin/posts/",
            get(handlers::post_handlers::serve_admin_page_index_handler)
        )
        .route(
            "/admin/posts/:id",
            get(handlers::post_handlers::serve_admin_page_id_handler)
        )
        .route(
            "/",
            get(handlers::index_handler::serve_index_page_handler),
        )
        .route("/mario", get(handlers::mario_index_handler::serve_index_page_handler))
        .route(
            "/api/hello",
            get(handlers::api_handlers::hello_json_api_handler),
        )
        .route(
            "/api/posts",
            post(handlers::post_handlers::create_post_handler)
                .get(handlers::post_handlers::get_posts_handler),
        )
        .route("/rpc", get(handlers::rpc_handlers::rpc_handler))
        .fallback_service(static_files_service)
        .with_state(app_state);
    println!("Axum router configured.");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server listening on http://{}", addr);

    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("FATAL: Could not bind to address {}: {:?}", addr, e);
            ::std::process::exit(1);
        }
    };

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        eprintln!("FATAL: Server error: {:?}", e);
        ::std::process::exit(1);
    }
}