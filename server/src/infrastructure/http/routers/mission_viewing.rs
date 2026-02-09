use std::sync::Arc;

use axum::{
    Json, Router, Extension,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    middleware,
    routing::get,
};

use crate::{
    application::use_cases::mission_viewing::MissionViewingUseCase,
    domain::{
        repositories::mission_viewing::MissionViewingRepository,
        value_objects::mission_filter::MissionFilter,
    },
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad, repositories::mission_viewing::MissionViewingPostgres,
        },
        http::middlewares::auth::auth,
    },
};

pub async fn get_one<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_one(mission_id, brawler_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_crew<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_crew(mission_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_all<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
    filter: Query<MissionFilter>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_all(&filter, brawler_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),

        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_joined<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_joined_missions(brawler_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn get_popular<T>(
    State(user_case): State<Arc<MissionViewingUseCase<T>>>,
    Extension(brawler_id): Extension<i32>,
) -> impl IntoResponse
where
    T: MissionViewingRepository + Send + Sync,
{
    match user_case.get_popular_missions(brawler_id).await {
        Ok(model) => (StatusCode::OK, Json(model)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let viewing_repositiory = MissionViewingPostgres::new(Arc::clone(&db_pool));
    let user_case = MissionViewingUseCase::new(Arc::new(viewing_repositiory));

    Router::new()
        .route("/popular", get(get_popular::<MissionViewingPostgres>))
        .route("/filter", get(get_all::<MissionViewingPostgres>))
        .route("/joined", get(get_joined::<MissionViewingPostgres>))
        .route("/crew/{mission_id}", get(get_crew::<MissionViewingPostgres>))
        .route("/{mission_id}", get(get_one::<MissionViewingPostgres>))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(user_case))
}
