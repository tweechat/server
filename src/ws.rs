use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::Response,
};

use crate::State;

#[allow(clippy::unused_async)]
pub async fn upgrade(ws: WebSocketUpgrade, state: State) -> Response {
    ws.on_upgrade(move |socket| async { handle_socket(socket, state) })
}

async fn handle_socket(mut socket: WebSocket, state: State) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            if msg.to_text().unwrap_or("disconnect").contains("disconnect") {}
        } else {
            // client disconnected
            return;
        };

        if socket.send(Message::Text("".to_string())).await.is_err() {
            // client disconnected
            return;
        }
    }
}
