use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{Ok, Result};
use axum::{
    Extension, Router, http::{
        Method, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    }
};
use tokio::{net::TcpListener, sync::broadcast};
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::info;

use crate::{
    config::config_model::DotEnvyConfig,
    domain::{entities::notification::Notification,
        services::notification_service::{NotificationService},
    },
    infrastructure::{
        database::postgresql_connection::PgPoolSquad,
        http::routers::{self},
        services::{notification_service::NotificationServiceImpl}},
    application::services::mission_realtime::MissionRealtimeService,
};



fn api_serve(
    db_pool: Arc<PgPoolSquad>,
    notification_service: Arc<dyn NotificationService>,
    tx: broadcast::Sender<Notification>,
    realtime_service: Arc<MissionRealtimeService>,
) -> Router {
    Router::new()
        .nest("/brawler", routers::brawlers::routes(Arc::clone(&db_pool)))
        .nest(
            "/view",
            routers::mission_viewing::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/mission",
            routers::mission_operation::routes(Arc::clone(&db_pool), Arc::clone(&notification_service), Arc::clone(&realtime_service))
        )
        .nest(
            "/crew",
            routers::crew_operation::routes(Arc::clone(&db_pool), Arc::clone(&notification_service), Arc::clone(&realtime_service))
        )
        .nest(
            "/mission-chat",
            routers::mission_chat::routes(Arc::clone(&db_pool), Arc::clone(&realtime_service)),
        )
        .nest(
            "/ws/mission",
            routers::mission_ws::routes(Arc::clone(&db_pool))
                .layer(Extension(realtime_service.clone())),
        )
        .nest(
            "/mission-management",
            routers::mission_management::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/authentication",
            routers::authentication::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/achievements",
            routers::achievements::routes(Arc::clone(&db_pool)),
        )
        .nest(
            "/notifications",
            routers::notifications::routes(tx),
        )
        .nest(
            "/mission-invites",
            routers::mission_invites::routes(Arc::clone(&db_pool), Arc::clone(&realtime_service)),
        )
        .nest("/util", routers::default_router::routes())
        .fallback(|| async { (StatusCode::NOT_FOUND, "API not found") })
}

pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPoolSquad>) -> Result<()> {
    let (tx, _rx) = broadcast::channel(100);
    let notification_svc: Arc<dyn NotificationService> = Arc::new(NotificationServiceImpl::new(tx.clone()));
    let realtime_svc = Arc::new(MissionRealtimeService::new());

    let dir = "statics";
    let static_service = ServeDir::new(dir).not_found_service(ServeFile::new(format!("{dir}/index.html")));

    let app = Router::new()
        .nest("/api", api_serve(db_pool, notification_svc, tx, realtime_svc))
        .fallback_service(static_service)
        .layer(tower_http::timeout::TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(config.server.timeout),
        ))
        .layer(RequestBodyLimitLayer::new(
            (config.server.body_limit * 1024 * 1024).try_into()?,
        ))
        .layer(
            CorsLayer::new()
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                    Method::OPTIONS,
                    Method::HEAD,
                ])
                .allow_origin(Any)
                .allow_headers([AUTHORIZATION, CONTENT_TYPE]),
        )
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    let listener = TcpListener::bind(addr).await?;

    info!("Server start on port {}", config.server.port);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async { tokio::signal::ctrl_c().await.expect("Fail ctrl + c") };

    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Receive ctrl + c signal"),
        _ = terminate => info!("Receive terminate signal"),
    }
}
