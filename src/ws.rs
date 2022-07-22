use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    headers::{authorization::Bearer, Authorization},
    response::Response,
    TypedHeader,
};
use tokio_stream::StreamExt;

use redis_subscribe::Message as rsm;
use tweechat_datatypes::User;

use crate::{auth::authenticate, Error, State};

pub async fn upgrade(
    ws: WebSocketUpgrade,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    state: State,
) -> Result<Response, Error> {
    let user = authenticate(auth, &state).await?;
    Ok(ws.on_upgrade(move |socket| async { handle_socket(socket, user, state).await }))
}

async fn handle_socket(mut socket: WebSocket, user: User, state: State) {
    socket
        .send(Message::Text("Connected".to_string()))
        .await
        .unwrap();
    let redis_subscriber = state.subscriber.clone();
    tokio::spawn(async move {
        let mut stream = redis_subscriber
            .listen()
            .await
            .expect("failed to connect to Redis");
        while let Some(redis_message) = stream.next().await {
            let message = match redis_message {
                rsm::Message {
                    channel: _,
                    message,
                }
                | rsm::PatternMessage {
                    pattern: _,
                    channel: _,
                    message,
                } => message,
                rsm::Disconnected(e) => format!("Disconnected from Redis: {:?}", e),
                rsm::Error(_) => "Error!".to_string(),
                _ => continue,
            };
            let msg = Message::Text(message.to_string());
            println!("Sending message {:?}", msg);
            if socket.send(msg.clone()).await.is_err() {
                // client disconnected
                return;
            };
        }
        state
            .subscriber
            .subscribe(format!("sends:{}", user.id))
            .await
            .unwrap();
    });
}
