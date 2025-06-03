use axum::{
    extract::{Path, State, Form, ws::{WebSocket, Message}, WebSocketUpgrade},
    response::{Html, IntoResponse, Redirect, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::Context;
use tokio::sync::broadcast::Receiver;

use crate::AppState;

#[derive(Serialize, Deserialize, Clone)]
pub struct Counter {
    pub id: String,
    pub value: i64,
}

#[derive(Deserialize)]
pub struct CounterAction {
    pub action: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CounterUpdate {
    pub id: String,
    pub count: i64,
}

pub async fn serve_counter_page(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let counter: Counter = match db.select(("counter", &id)).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            let c = Counter { id: id.clone(), value: 0 };
            let _ : Result<Option<Counter>, _> = db.create(("counter", &id)).content(c.clone()).await;
            c
        }
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response();
        }
    };

    let tera = &app_state.templates;
    let mut ctx = Context::new();
    ctx.insert("c", &counter);
    match tera.render("counter.html", &ctx) {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "template error").into_response()
        }
    }
}

pub async fn get_counter(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = &app_state.db;
    match db.select(("counter", &id)).await {
        Ok(Some(c)) => Json::<Counter>(c).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response()
        }
    }
}

pub async fn update_counter(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Form(form): Form<CounterAction>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let mut counter: Counter = match db.select(("counter", &id)).await {
        Ok(Some(c)) => c,
        Ok(None) => Counter { id: id.clone(), value: 0 },
        Err(e) => {
            eprintln!("DB error: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response();
        }
    };
    match form.action.as_str() {
        "inc" => counter.value += 1,
        "dec" => counter.value -= 1,
        _ => {}
    }
    let res: Result<Option<Counter>, _> = db
        .update(("counter", &id))
        .content(counter.clone())
        .await;
    if let Err(e) = res {
        eprintln!("DB update error: {:?}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "db error").into_response();
    }
    let _ = app_state.counter_tx.send(CounterUpdate { id: counter.id.clone(), count: counter.value });
    Redirect::to(&format!("/counter/{}", counter.id)).into_response()
}

pub async fn ws_counter(
    ws: WebSocketUpgrade,
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let rx = app_state.counter_tx.subscribe();
    ws.on_upgrade(move |socket| handle_socket(socket, rx, id))
}

async fn handle_socket(mut socket: WebSocket, mut rx: Receiver<CounterUpdate>, id: String) {
    while let Ok(update) = rx.recv().await {
        if update.id != id {
            continue;
        }
        let msg = serde_json::to_string(&update).unwrap();
        if socket.send(Message::Text(msg)).await.is_err() {
            break;
        }
    }
}
