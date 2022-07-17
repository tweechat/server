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
    ws.on_upgrade(move |socket| async { handle_socket(socket, state).await })
}

async fn handle_socket(mut socket: WebSocket, state: State) {
    socket.send(Message::Text("Connected".to_string()));
    loop {
        redis
        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
