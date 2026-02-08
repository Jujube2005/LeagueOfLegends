use axum::{
    extract::State,
    response::sse::{Event, KeepAlive, Sse},
    routing::get,
    Router, Extension, middleware,
};
use futures::stream::Stream;
use std::{convert::Infallible, time::Duration};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;
use tracing::info;

use crate::{
    domain::entities::notification::Notification,
    infrastructure::http::middlewares::auth::auth,
};

pub fn routes(tx: broadcast::Sender<Notification>) -> Router {
    Router::new()
        .route("/events", get(sse_handler))
        .route_layer(middleware::from_fn(auth))
        .with_state(tx)
}

async fn sse_handler(
    State(tx): State<broadcast::Sender<Notification>>,
    Extension(user_id): Extension<i32>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    info!("SSE connection established for user: {}", user_id);
    let rx = tx.subscribe();
    let stream = BroadcastStream::new(rx);

    // Create a welcome message
    let welcome_msg = Event::default().data(serde_json::json!({
        "title": "System",
        "message": "Connected to notification server",
        "notification_type": "System",
        "metadata": {},
        "recipient_id": user_id
    }).to_string());

    let welcome_stream = tokio_stream::iter(vec![Ok(welcome_msg)]);

    let main_stream = stream.map(move |msg| {
        match msg {
            Ok(notification) => {
                // Filter messages
                if notification.recipient_id.is_none() || notification.recipient_id == Some(user_id) {
                    info!("Sending notification to user {}: {:?}", user_id, notification.title);
                    let data = serde_json::to_string(&notification).unwrap_or_default();
                    Event::default().data(data)
                } else {
                    // Send a comment as a heartbeat/noop for filtered messages
                    Event::default().comment("ignore")
                }
            }
            Err(e) => {
                info!("Broadcast stream error for user {}: {:?}", user_id, e);
                Event::default().comment("missed message")
            },
        }
    }).map(Ok);

    Sse::new(welcome_stream.chain(main_stream))
        .keep_alive(KeepAlive::new().interval(Duration::from_secs(15)).text("keep-alive"))
}
