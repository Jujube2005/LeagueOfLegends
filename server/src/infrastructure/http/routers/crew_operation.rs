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
        repositories::{
            crew_operation::CrewOperationRepository, mission_viewing::MissionViewingRepository,
            AchievementRepository, BrawlerRepository,
        },
        services::notification_service::NotificationService,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                crew_operation::CrewOperationPostgres, mission_viewing::MissionViewingPostgres,
                achievements::AchievementRepositoryImpl, brawlers::BrawlerPostgres,
            },
        },
        http::middlewares::auth::auth,
    },
};

pub async fn join<T1, T2, T3, T4>(
    State(user_case): State<Arc<CrewOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.join(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": format!("Join Mission_id:{} completed", mission_id) })),
        )
            .into_response(),

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
) -> impl IntoResponse
where
    T1: CrewOperationRepository + Send + Sync + 'static,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.leave(mission_id, user_id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": format!("Leave Mission_id:{} completed", mission_id) })),
        )
            .into_response(),

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
    let brawler_repository = BrawlerPostgres::new(Arc::clone(&db_pool));
    
    let user_case = CrewOperationUseCase::new(
        Arc::new(crew_operation_repository),
        Arc::new(viewing_repositiory),
        Arc::new(achievement_repository),
        Arc::new(brawler_repository),
        notification_service,
    );

    Router::new()
        .route(
            "/join/{mission_id}",
            post(join::<CrewOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>),
        )
        .route(
            "/leave/{mission_id}",
            delete(leave::<CrewOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>),
        )
        .route(
            "/kick/{mission_id}",
            post(kick::<CrewOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>),
        )
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
