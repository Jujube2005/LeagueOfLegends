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

use crate::application::services::mission_realtime::{MissionRealtimeService, ChatMessage};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(mission_id): Path<i32>,
    Extension(user_id): Extension<i32>,
    State(db_pool): State<Arc<PgPoolSquad>>,
    Extension(realtime_service): Extension<Arc<MissionRealtimeService>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, mission_id, user_id, realtime_service, db_pool))
}

async fn handle_socket(
    socket: WebSocket,
    mission_id: i32,
    user_id: i32,
    realtime_service: Arc<MissionRealtimeService>,
    db_pool: Arc<PgPoolSquad>,
) {
    let (mut sender, mut receiver) = socket.split();

    let tx = realtime_service.get_channel(mission_id);
    let mut rx = tx.subscribe();

    let message_repo = MissionMessagePostgres::new(db_pool);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let payload = json!({
                "id": 0, // In broadcast we usually don't have the saved ID easily unless we pass it
                "user_id": msg.user_id,
                "user_display_name": msg.user_display_name,
                "user_avatar_url": msg.user_avatar_url,
                "content": msg.content,
                "type": msg.type_,
                "created_at": msg.created_at
            }).to_string();

            if sender.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    let rt_service_for_recv = Arc::clone(&realtime_service);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                let content = text.clone();
                let entity = NewMissionMessageEntity {
                    mission_id,
                    user_id: Some(user_id),
                    content: content.to_string(),
                    type_: "chat".to_string(),
                };

                match message_repo.create(entity).await {
                    Ok(saved_msg) => {
                         // Broadcast via structured service
                        let broadcast_msg = ChatMessage {
                            mission_id,
                            user_id: Some(user_id),
                            user_display_name: None, // Could fetch if needed
                            user_avatar_url: None,
                            content: content.to_string(),
                            type_: "chat".to_string(),
                            created_at: chrono::Utc::now().to_rfc3339(),
                        };
                        rt_service_for_recv.broadcast(mission_id, broadcast_msg);
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
        .route("/{mission_id}", get(ws_handler))
        .route_layer(middleware::from_fn(auth))
        .with_state(db_pool)
}
