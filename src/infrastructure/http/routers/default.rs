use axum ::{Router, extract::Path, http::StatusCode, response::IntoResponse};

use axum::routing::get;

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK").into_response()
}

pub async fn make_error(Path(code): Path<u16>) -> impl IntoResponse {
    let status_code = StatusCode::from_u16(code).unwrap();
    (status_code, code.to_string()).into_response()
}

pub fn routes() -> Router {
    Router::new()
        .route("/make-error/{code}", get(make_error))
        .route("/health-check", get(health_check)) 
}



