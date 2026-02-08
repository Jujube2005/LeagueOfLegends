use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::{config::config_loader::get_jwt_env, infrastructure::jwt::verify_token};

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let token = if let Some(header) = req.headers().get(header::AUTHORIZATION) {
        header
            .to_str()
            .ok()
            .and_then(|value| value.strip_prefix("Bearer "))
            .map(|s| s.to_string())
    } else {
        // Fallback: Check for token in query parameters (for EventSource)
        req.uri()
            .query()
            .and_then(|q| {
                q.split('&')
                    .find(|param| param.starts_with("token="))
                    .map(|param| param.trim_start_matches("token=").to_string())
            })
    };

    let token = token.ok_or(StatusCode::UNAUTHORIZED)?;

    let jwt_env = get_jwt_env().unwrap();
    let secret = jwt_env.secret;

    let claims = verify_token(secret, token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user_id = claims
        .sub
        .parse::<i32>()
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(user_id);

    Ok(next.run(req).await)
}
