use std::sync::Arc;

use axum::{
    Extension, Router,
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::patch,
};

use crate::{
    application::use_cases::mission_operation::MissionOperationUseCase,
    domain::{
        repositories::{
            mission_operation::MissionOperationRepository, mission_viewing::MissionViewingRepository,
            AchievementRepository, BrawlerRepository,
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
            },
        },
        http::middlewares::auth::auth,
    },
};

pub async fn in_progress<T1, T2, T3, T4>(
    State(user_case): State<Arc<MissionOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.in_progress(mission_id, user_id).await {
        Ok(mission_id) => (StatusCode::OK, mission_id.to_string()).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_completed<T1, T2, T3, T4>(
    State(user_case): State<Arc<MissionOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.to_completed(mission_id, user_id).await {
        Ok(mission_id) => (StatusCode::OK, mission_id.to_string()).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn to_failed<T1, T2, T3, T4>(
    State(user_case): State<Arc<MissionOperationUseCase<T1, T2, T3, T4>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T1: MissionOperationRepository + Send + Sync,
    T2: MissionViewingRepository + Send + Sync,
    T3: AchievementRepository + Send + Sync,
    T4: BrawlerRepository + Send + Sync,
{
    match user_case.to_failed(mission_id, user_id).await {
        Ok(mission_id) => (StatusCode::OK, mission_id.to_string()).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>, notification_service: Arc<dyn NotificationService>) -> Router {
    let mission_repository = MissionOperationPostgres::new(Arc::clone(&db_pool));
    let viewing_repositiory = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let achievement_repository = AchievementRepositoryImpl::new(Arc::clone(&db_pool));
    let brawler_repository = BrawlerPostgres::new(Arc::clone(&db_pool));

    let user_case = MissionOperationUseCase::new(
        Arc::new(mission_repository),
        Arc::new(viewing_repositiory),
        Arc::new(achievement_repository),
        Arc::new(brawler_repository),
        notification_service,
    );

    Router::new()
        .route("/in-progress/{mission_id}", patch(in_progress::<MissionOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>))
        .route("/to-completed/{mission_id}", patch(to_completed::<MissionOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>))
        .route("/to-failed/{mission_id}", patch(to_failed::<MissionOperationPostgres, MissionViewingPostgres, AchievementRepositoryImpl, BrawlerPostgres>))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
