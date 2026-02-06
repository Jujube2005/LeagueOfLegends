use std::sync::Arc;
use axum::{
    extract::{State, Extension},
    response::IntoResponse,
    http::StatusCode,
    Json,
    Router,
    routing::get,
    middleware,
};
use crate::{
    application::use_cases::achievements::AchievementUseCase,
    domain::repositories::AchievementRepository,
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::achievements::AchievementRepositoryImpl,
        },
        http::middlewares::auth::auth,
    },
};

pub async fn get_my_achievements<T>(
    State(use_case): State<Arc<AchievementUseCase<T>>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse
where
    T: AchievementRepository + Send + Sync,
{
    match use_case.get_my_achievements(user_id).await {
        Ok(achievements) => Json(achievements).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = AchievementRepositoryImpl::new(db_pool);
    let use_case = AchievementUseCase::new(Arc::new(repository));

    Router::new()
        .route("/", get(get_my_achievements))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(use_case))
}
