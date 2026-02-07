use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State, Extension,
    },
    response::IntoResponse,
    routing::get,
    Router,
    middleware,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use chrono;

use crate::{
    domain::{
        entities::mission_messages::NewMissionMessageEntity,
        repositories::mission_message_repository::MissionMessageRepository,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::mission_messages::MissionMessagePostgres,
        },
        services::mission_websocket_service::MissionWebSocketService,
        http::middlewares::auth::auth,
    },
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(mission_id): Path<i32>,
    Extension(user_id): Extension<i32>,
    State(db_pool): State<Arc<PgPoolSquad>>,
    Extension(ws_service): Extension<Arc<MissionWebSocketService>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, mission_id, user_id, ws_service, db_pool))
}

async fn handle_socket(
    socket: WebSocket,
    mission_id: i32,
    user_id: i32,
    ws_service: Arc<MissionWebSocketService>,
    db_pool: Arc<PgPoolSquad>,
) {
    let (mut sender, mut receiver) = socket.split();

    let tx = ws_service.get_or_create_room(mission_id).await;
    let mut rx = tx.subscribe();

    let message_repo = MissionMessagePostgres::new(db_pool);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                // Determine if it's a command or chat message?
                // For now, assume everything is chat.
                
                let content = text.clone();
                let entity = NewMissionMessageEntity {
                    mission_id,
                    user_id: Some(user_id),
                    content: content.clone(),
                    type_: "chat".to_string(),
                };

                match message_repo.create(entity).await {
                    Ok(saved_msg) => {
                         // Broadcast the saved message with metadata
                        let broadcast_payload = json!({
                            "id": saved_msg.id, // Include ID for key-ing
                            "mission_id": mission_id,
                            "user_id": user_id,
                            "content": content,
                            "type": "chat",
                            "created_at": chrono::Utc::now().to_rfc3339() // Should use saved_msg time if available
                        }).to_string();
                        
                        let _ = tx.send(broadcast_payload);
                    },
                    Err(e) => {
                        eprintln!("Failed to save message: {}", e);
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    Router::new()
        .route("/:mission_id", get(ws_handler))
        .route_layer(middleware::from_fn(auth))
        .with_state(db_pool)
}
