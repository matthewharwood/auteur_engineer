use axum::{
    extract::{ws::{Message, WebSocket}, State, WebSocketUpgrade},
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client as WsClient;

use crate::AppState;

pub async fn rpc_handler(
    State(app_state): State<Arc<AppState>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let db = app_state.db.clone();
    ws.on_upgrade(move |socket| handle_ws(socket, db))
}

async fn handle_ws(mut socket: WebSocket, db: Arc<Surreal<WsClient>>) {
    let mut stream = db
        .select("posts")
        .live()
        .await
        .unwrap();

    while let Some(Ok(notification)) = stream.next().await {
        let txt = serde_json::to_string(&(notification.action, notification.data)).unwrap();
        if socket.send(Message::Text(txt)).await.is_err() {
            break;
        }
    }
}
