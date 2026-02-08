use std::sync::Arc;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};

use crate::{
    application::use_cases::authentication::AuthenticationUseCase,
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
        jwt::authentication_model::{LoginModel, RecoverPasswordModel},
    },
};

pub async fn login<T>(
    State(user_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(model): Json<LoginModel>,
) -> impl IntoResponse
where
    T: crate::domain::repositories::brawlers::BrawlerRepository + Send + Sync,
{
    match user_case.login(model).await {
        Ok(passport) => (StatusCode::OK, Json(passport)).into_response(),

        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub async fn recover_password<T>(
    State(user_case): State<Arc<AuthenticationUseCase<T>>>,
    Json(model): Json<RecoverPasswordModel>,
) -> impl IntoResponse
where
    T: crate::domain::repositories::brawlers::BrawlerRepository + Send + Sync,
{
    match user_case.recover_password(model).await {
        Ok(msg) => (StatusCode::OK, Json(serde_json::json!({ "message": msg }))).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    }
}

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let repository = BrawlerPostgres::new(db_pool);
    let user_case = AuthenticationUseCase::new(Arc::new(repository));

    Router::new()
        .route("/login", post(login))
        // .route("/recover-password", post(recover_password))
        .with_state(Arc::new(user_case))
}
