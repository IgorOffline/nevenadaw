mod error;

use crate::error::Error;
use axum::{http::header::HeaderName, http::StatusCode, routing::get, Router};
use fjall::{Database, Keyspace};

#[allow(dead_code)]
#[derive(Clone)]
struct State {
    db: Database,
    tree: Keyspace,
}

#[allow(dead_code)]
#[tokio::main]
pub async fn data_main() -> Result<(), Error> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".into())
        .parse::<u16>()
        .expect("invalid port");

    log::info!("Opening database");

    let db = Database::builder(".fjall_data").open()?;
    let tree = db.keyspace("data", fjall::KeyspaceCreateOptions::default)?;

    let state = State { db, tree };

    log::info!("Starting on port {port}");

    let app = Router::new()
        .route("/folder_exists", get(folder_exists))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&format!("0.0.0.0:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[allow(dead_code)]
async fn folder_exists() -> Result<(StatusCode, [(HeaderName, String); 1], &'static str), Error> {
    log::debug!("folder_exists");

    let before = std::time::Instant::now();

    Ok((
        StatusCode::OK,
        [(
            HeaderName::from_static("x-took-ms"),
            before.elapsed().as_millis().to_string(),
        )],
        StatusCode::OK.as_str(),
    ))
}
