use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, post},
};
use serde_json::json;

use crate::{
    application::use_cases::crew_operation::CrewOperationUseCase,
    domain::{
        entities::mission_messages::NewMissionMessageEntity,
        repositories::{
            crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
            AchievementRepository, BrawlerRepository, mission_message_repository::MissionMessageRepository,
        },
        services::notification_service::NotificationService,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                crew_operation::CrewOperationPostgres, mission_viewing::MissionViewingPostgres,
                achievements::AchievementRepositoryImpl, brawlers::BrawlerPostgres,
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

pub async fn join<T1, T2, T3, T4>(
    State(user_case): State<Arc<CrewOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Extension(ws_service): Extension<Arc<MissionWebSocketService>>,
    Extension(message_repo): Extension<Arc<MissionMessagePostgres>>,
    Extension(brawler_repo): Extension<Arc<BrawlerPostgres>>, // Use concrete type for simplicity or T4? T4 is generic.
    // To get username, we need BrawlerRepository. T4 is BrawlerRepository but we can't access it easily via UseCase.
    // Let's pass BrawlerPostgres as Extension.
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.join(mission_id, user_id).await {
        Ok(_) => {
            // Get user name for message
            let username = match brawler_repo.find_by_id(user_id).await {
                Ok(brawler) => brawler.display_name,
                Err(_) => "Unknown".to_string(),
            };

            broadcast_system_message(
                mission_id, 
                format!("{} joined the mission", username), 
                message_repo.as_ref(), 
                &ws_service
            ).await;

            (
                StatusCode::OK,
                Json(json!({ "message": format!("Join Mission_id:{} completed", mission_id) })),
            )
            .into_response()
        },

        Err(e) => {
            let error_message = e.to_string();
            let status = if error_message.contains("Already joined") {
                StatusCode::CONFLICT
            } else if error_message.contains("Mission is full") {
                StatusCode::CONFLICT
            } else if error_message.contains("Mission is not joinable") {
                StatusCode::BAD_REQUEST
            } else if error_message.contains("The Chief can not join") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            (status, Json(json!({ "message": error_message }))).into_response()
        }
    }
}

pub async fn leave<T1, T2, T3, T4>(
    State(user_case): State<Arc<CrewOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Extension(ws_service): Extension<Arc<MissionWebSocketService>>,
    Extension(message_repo): Extension<Arc<MissionMessagePostgres>>,
    Extension(brawler_repo): Extension<Arc<BrawlerPostgres>>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.leave(mission_id, user_id).await {
        Ok(_) => {
             // Get user name for message
            let username = match brawler_repo.find_by_id(user_id).await {
                Ok(brawler) => brawler.display_name,
                Err(_) => "Unknown".to_string(),
            };

            broadcast_system_message(
                mission_id, 
                format!("{} left the mission", username), 
                message_repo.as_ref(), 
                &ws_service
            ).await;

            (
                StatusCode::OK,
                Json(json!({ "message": format!("Leave Mission_id:{} completed", mission_id) })),
            )
            .into_response()
        },

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": e.to_string() })),
        )
            .into_response(),
    }
}

#[derive(serde::Deserialize)]
pub struct KickModel {
    member_id: i32,
}

pub async fn kick<T1, T2, T3, T4>(
    State(user_case): State<Arc<CrewOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(model): Json<KickModel>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case
        .kick_crew(mission_id, user_id, model.member_id)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(json!({ "message": "Member kicked" }))).into_response(),
        Err(e) => {
            let error_message = e.to_string();
            let status = if error_message.contains("Only the Chief can kick members") {
                StatusCode::FORBIDDEN
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(json!({ "message": error_message }))).into_response()
        }
    }
}

pub fn routes(
    db_pool: Arc<PgPoolSquad>,
    notification_service: Arc<dyn NotificationService>,
) -> Router {
    let crew_operation_repository = CrewOperationPostgres::new(Arc::clone(&db_pool));
    let viewing_repositiory = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let achievement_repository = AchievementRepositoryImpl::new(Arc::clone(&db_pool));
    // Wrap in Arc here to share with Extension
    let brawler_repository = Arc::new(BrawlerPostgres::new(Arc::clone(&db_pool)));
    let mission_message_repository = Arc::new(MissionMessagePostgres::new(Arc::clone(&db_pool)));
    
    let user_case = CrewOperationUseCase::new(
        Arc::new(crew_operation_repository),
        Arc::new(viewing_repositiory),
        Arc::new(achievement_repository),
        Arc::clone(&brawler_repository),
        notification_service,
    );


    Router::new()
        .route(
            "/join/:mission_id",
            post(join::<CrewOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>),
        )
        .route(
            "/leave/:mission_id",
            delete(leave::<CrewOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>),
        )
        .route(
            "/kick/:mission_id",
            post(kick::<CrewOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>),
        )
        .layer(Extension(mission_message_repository))
        .layer(Extension(brawler_repository))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
