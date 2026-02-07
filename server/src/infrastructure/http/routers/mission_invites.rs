
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode as AxumStatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router, middleware,
};
use std::sync::Arc;
use serde::Deserialize;

use crate::{
    application::use_cases::mission_invites::MissionInviteUseCase,
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::{
                achievements::AchievementRepositoryImpl, brawlers::BrawlerPostgres,
                crew_operation::CrewOperationPostgres,
                mission_invites::MissionInvitePostgres,
                mission_messages::MissionMessagePostgres,
                mission_viewing::MissionViewingPostgres,
            },
        },
        http::middlewares::auth::auth,
    },
};

#[derive(Deserialize)]
pub struct InviteUserPayload {
    user_id: i32,
}

async fn invite_user(
    State(use_case): State<Arc<MissionInviteUseCase>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(payload): Json<InviteUserPayload>,
) -> impl IntoResponse {
    match use_case
        .invite(mission_id, user_id, payload.user_id)
        .await
    {
        Ok(invite) => (AxumStatusCode::OK, Json(invite)).into_response(),
        Err(e) => (AxumStatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn accept_invite(
    State(use_case): State<Arc<MissionInviteUseCase>>,
    Extension(user_id): Extension<i32>,
    Path(invite_id): Path<i32>,
) -> impl IntoResponse {
    match use_case.accept(invite_id, user_id).await {
        Ok(_) => (AxumStatusCode::OK, "Invite accepted").into_response(),
        Err(e) => (AxumStatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn decline_invite(
    State(use_case): State<Arc<MissionInviteUseCase>>,
    Extension(user_id): Extension<i32>,
    Path(invite_id): Path<i32>,
) -> impl IntoResponse {
    match use_case.decline(invite_id, user_id).await {
        Ok(_) => (AxumStatusCode::OK, "Invite declined").into_response(),
        Err(e) => (AxumStatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

async fn get_my_invites(
    State(use_case): State<Arc<MissionInviteUseCase>>,
    Extension(user_id): Extension<i32>,
) -> impl IntoResponse {
    match use_case.get_my_pending_invites(user_id).await {
        Ok(invites) => (AxumStatusCode::OK, Json(invites)).into_response(),
        Err(e) => (AxumStatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let invite_repo = Arc::new(MissionInvitePostgres::new(Arc::clone(&db_pool)));
    let mission_repo = Arc::new(MissionViewingPostgres::new(Arc::clone(&db_pool)));
    let crew_repo = Arc::new(CrewOperationPostgres::new(Arc::clone(&db_pool)));
    let brawler_repo = Arc::new(BrawlerPostgres::new(Arc::clone(&db_pool)));
    let message_repo = Arc::new(MissionMessagePostgres::new(Arc::clone(&db_pool)));
    let achievement_repo = Arc::new(AchievementRepositoryImpl::new(Arc::clone(&db_pool)));

    let use_case = Arc::new(MissionInviteUseCase::new(
        invite_repo,
        mission_repo,
        crew_repo,
        brawler_repo,
        message_repo,
        achievement_repo,
    ));

    Router::new()
        .route("/mission/:mission_id/invite", post(invite_user))
        .route("/invite/:invite_id/accept", post(accept_invite))
        .route("/invite/:invite_id/decline", post(decline_invite))
        .route("/my-invites", get(get_my_invites))
        .route_layer(middleware::from_fn(auth))
        .with_state(use_case)
}
