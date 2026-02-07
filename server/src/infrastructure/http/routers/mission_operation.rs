use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
};
use serde_json::json;

use crate::{
    application::use_cases::mission_operation::MissionOperationUseCase,
    domain::{
        entities::mission_messages::NewMissionMessageEntity,
        repositories::{
            mission_operation::MissionOperationRepository, mission_viewing::MissionViewingRepository,
            AchievementRepository, BrawlerRepository, mission_message_repository::MissionMessageRepository,
        },
        services::notification_service::NotificationService,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                mission_operation::MissionOperationPostgres,
                mission_viewing::MissionViewingPostgres,
                achievements::AchievementRepositoryImpl,
                brawlers::BrawlerPostgres,
                mission_messages::MissionMessagePostgres,
            },
        },
        http::middlewares::auth::auth,
        services::mission_websocket_service::MissionWebSocketService,
    },
};

async fn broadcast_system_message(
    mission_id: i32,
    content: String,
    message_repo: &impl MissionMessageRepository,
    ws_service: &MissionWebSocketService,
) {
    let entity = NewMissionMessageEntity {
        mission_id,
        user_id: None, // System message
        content: content.clone(),
        type_: "system".to_string(),
    };

    if let Ok(_) = message_repo.create(entity).await {
         let broadcast_msg = json!({
            "user_id": null,
            "content": content,
            "type": "system",
            "created_at": chrono::Utc::now().to_rfc3339()
         }).to_string();
         ws_service.broadcast(mission_id, broadcast_msg).await;
    }
}

pub async fn in_progress<T1, T2, T3, T4>(
    State(user_case): State<Arc<MissionOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Extension(ws_service): Extension<Arc<MissionWebSocketService>>,
    Extension(message_repo): Extension<Arc<MissionMessagePostgres>>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.in_progress(mission_id, user_id).await {
        Ok(mission_id) => {
            broadcast_system_message(
                mission_id,
                "Mission started".to_string(),
                message_repo.as_ref(),
                &ws_service
            ).await;
            (StatusCode::OK, mission_id.to_string()).into_response()
        },

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_completed<T1, T2, T3, T4>(
    State(user_case): State<Arc<MissionOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Extension(ws_service): Extension<Arc<MissionWebSocketService>>,
    Extension(message_repo): Extension<Arc<MissionMessagePostgres>>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.to_completed(mission_id, user_id).await {
        Ok(mission_id) => {
            broadcast_system_message(
                mission_id,
                "Mission completed".to_string(),
                message_repo.as_ref(),
                &ws_service
            ).await;
            (StatusCode::OK, mission_id.to_string()).into_response()
        },

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_failed<T1, T2, T3, T4>(
    State(user_case): State<Arc<MissionOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Extension(ws_service): Extension<Arc<MissionWebSocketService>>,
    Extension(message_repo): Extension<Arc<MissionMessagePostgres>>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.to_failed(mission_id, user_id).await {
        Ok(mission_id) => {
             broadcast_system_message(
                mission_id,
                "Mission failed".to_string(),
                message_repo.as_ref(),
                &ws_service
            ).await;
            (StatusCode::OK, mission_id.to_string()).into_response()
        },

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>, notification_service: Arc<dyn NotificationService>) -> Router {
    let mission_repository = MissionOperationPostgres::new(Arc::clone(&db_pool));
    let viewing_repositiory = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let achievement_repository = AchievementRepositoryImpl::new(Arc::clone(&db_pool));
    let brawler_repository = BrawlerPostgres::new(Arc::clone(&db_pool));
    let mission_message_repository = Arc::new(MissionMessagePostgres::new(Arc::clone(&db_pool)));

    let user_case = MissionOperationUseCase::new(
        Arc::new(mission_repository),
        Arc::new(viewing_repositiory),
        Arc::new(achievement_repository),
        Arc::new(brawler_repository),
        notification_service,
    );

    Router::new()
        .route("/in-progress/:mission_id", patch(in_progress::<MissionOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>))
        .route("/to-completed/:mission_id", patch(to_completed::<MissionOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>))
        .route("/to-failed/:mission_id", patch(to_failed::<MissionOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>))
        .layer(Extension(mission_message_repository))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
