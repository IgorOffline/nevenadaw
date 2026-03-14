use axum::{routing::get, Json, Router};
use chrono::{DateTime, Utc};
use rand::RngExt;
use serde::Serialize;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
struct NevenaMail {
    id: i32,

    #[serde(serialize_with = "serialize_datetime")]
    time: DateTime<Utc>,
}

fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let formatted = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info,tower_http=debug,axum=debug"))
        .init();

    let mail = NevenaMail {
        id: rand::rng().random_range(1..=6),
        time: Utc::now(),
    };
    let payload = serde_json::to_string_pretty(&mail).unwrap();

    let port = 8000;
    println!(
        "Axum server running on port {} -- {} -- {}",
        port,
        payload,
        Uuid::new_v4()
    );

    let app = Router::new()
        .route(
            "/",
            get(|| async {
                Json(NevenaMail {
                    id: rand::rng().random_range(1..=6),
                    time: Utc::now(),
                })
            }),
        )
        .layer(TraceLayer::new_for_http());

    let address = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
