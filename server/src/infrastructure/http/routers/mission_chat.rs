use std::sync::Arc;
use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router, Extension, middleware,
};
use serde::Deserialize;

use crate::{
    application::use_cases::mission_chat::MissionChatUseCase,
    domain::repositories::mission_message_repository::MissionMessageRepository,
    infrastructure::{
        database::{
            postgresql_connection::PgPoolSquad,
            repositories::mission_messages::MissionMessagePostgres,
        },
        http::middlewares::auth::auth,
    },
};

#[derive(Deserialize)]
pub struct SendMessageDto {
    pub content: String,
}

pub async fn get_messages<T>(
    State(use_case): State<Arc<MissionChatUseCase<T>>>,
    Path(mission_id): Path<i32>,
) -> impl IntoResponse
where
    T: MissionMessageRepository + Send + Sync,
{
    match use_case.get_messages(mission_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn send_message<T>(
    State(use_case): State<Arc<MissionChatUseCase<T>>>,
    Extension(user_id): Extension<i32>,
    Path(mission_id): Path<i32>,
    Json(body): Json<SendMessageDto>,
) -> impl IntoResponse
where
    T: MissionMessageRepository + Send + Sync,
{
    match use_case.send_message(mission_id, user_id, body.content).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = MissionMessagePostgres::new(Arc::clone(&db_pool));
    let use_case = MissionChatUseCase::new(Arc::new(repository));

    Router::new()
        .route("/{mission_id}/messages", get(get_messages::<MissionMessagePostgres>))
        .route("/{mission_id}/messages", post(send_message::<MissionMessagePostgres>))
        .route_layer(middleware::from_fn(auth))
        .with_state(Arc::new(use_case))
}
