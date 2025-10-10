use std::sync::Arc;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket};
use futures_util::{SinkExt, StreamExt};
use serde_json::from_str;
use crate::game::{GameOptions, GameState};

pub async fn manage_pre_lobby(socket: WebSocket, state: Arc<GameState>) {
    let (client_id, mut rx) = state.register_client(GameOptions {
        amount: 10,
        category: None,
        difficulty: None,
        kind: None,
    });

    println!("Client connected: {client_id}");

    let (mut sender, mut receiver) = socket.split();

    let mut recv_task = {
        let state = Arc::clone(&state);
        let client_id = client_id.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                if let Message::Text(text) = msg {
                    if let Ok(new_opts) = from_str::<GameOptions>(&text) {
                        println!("New settings {client_id}: {:?}", new_opts);
                        state.broadcast(new_opts);
                    }
                }
            }
            println!("Client disconnected: {client_id}");
            state.unregister_client(&client_id);
        })
    };

    let mut send_task = tokio::spawn(async move {
        while rx.changed().await.is_ok() {
            let opts = rx.borrow().clone();
            if sender
                .send(Message::Text(Utf8Bytes::from(
                    serde_json::to_string(&opts).unwrap(),
                )))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    tokio::select! {
	_ = &mut recv_task => send_task.abort(),
	_ = &mut send_task => recv_task.abort(),
	}
}