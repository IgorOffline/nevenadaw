mod error;

use crate::error::Error;
use axum::routing::{get, post};
use axum::{extract, http::header::HeaderName, http::StatusCode, Json, Router};
use fjall::{Database, Keyspace, PersistMode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const CREATED_MESSAGE: &str = "CREATED";
const DATA_DIR: &str = ".fjall_data";
const DATA_KEYSPACE: &str = "data";
const DEFAULT_PORT: &str = "8000";
const DURATION_HEADER: &str = "x-took-ms";
const ERR_PORT: &str = "invalid port";
const GET_ALL_ROUTE: &str = "/items";
const INSERT_ITEM_ROUTE: &str = "/insert_item";
const OPENING_DATABASE_MESSAGE: &str = "Opening database";
const PORT_CONST: &str = "PORT";
const STARTING_ON_PORT_MESSAGE: &str = "Starting on port";
const TCP_LISTENER_PREFIX: &str = "0.0.0.0";

#[derive(Debug, Deserialize, Serialize)]
struct Payload {
    value: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct InsertBody {
    item: Payload,
}

#[derive(Clone)]
struct State {
    db: Database,
    tree: Keyspace,
}

#[tokio::main]
pub async fn data_main() -> Result<(), Error> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let port = std::env::var(PORT_CONST)
        .unwrap_or_else(|_| DEFAULT_PORT.into())
        .parse::<u16>()
        .expect(ERR_PORT);

    log::info!("{OPENING_DATABASE_MESSAGE}");

    let db = Database::builder(DATA_DIR).open()?;
    let tree = db.keyspace(DATA_KEYSPACE, fjall::KeyspaceCreateOptions::default)?;

    let state = State { db, tree };

    log::info!("{STARTING_ON_PORT_MESSAGE} {port}");

    let app = Router::new()
        .route(&format!("{INSERT_ITEM_ROUTE}/{{key}}"), post(insert_item))
        .route(GET_ALL_ROUTE, get(get_all_items))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&format!("{TCP_LISTENER_PREFIX}:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn insert_item(
    extract::Path(key): extract::Path<String>,
    extract::State(state): extract::State<State>,
    extract::Json(body): extract::Json<InsertBody>,
) -> Result<(StatusCode, [(HeaderName, String); 1], &'static str), Error> {
    log::debug!(
        "{INSERT_ITEM_ROUTE} {key} {}",
        serde_json::to_string_pretty(&body.item)?
    );

    let before = std::time::Instant::now();

    tokio::task::spawn_blocking(move || {
        state
            .tree
            .insert(key, serde_json::to_string(&body.item).unwrap())?;

        state.db.persist(PersistMode::SyncAll)
    })
    .await
    .unwrap()?;

    Ok((
        StatusCode::CREATED,
        [(
            HeaderName::from_static(DURATION_HEADER),
            before.elapsed().as_millis().to_string(),
        )],
        CREATED_MESSAGE,
    ))
}

async fn get_all_items(
    extract::State(state): extract::State<State>,
) -> Result<(StatusCode, [(HeaderName, String); 1], Json<Value>), Error> {
    log::debug!("{GET_ALL_ROUTE}");

    let before = std::time::Instant::now();

    let mut items = serde_json::Map::new();

    for item in state.tree.iter() {
        let (key_raw, val_raw) = item.into_inner()?;
        let key = String::from_utf8_lossy(&key_raw).to_string();
        let val_str = String::from_utf8_lossy(&val_raw);
        log::info!("{key} :: {val_str}");

        let val_json: Value = serde_json::from_str(&val_str)?;
        items.insert(key, val_json);
    }

    Ok((
        StatusCode::OK,
        [(
            HeaderName::from_static(DURATION_HEADER),
            before.elapsed().as_millis().to_string(),
        )],
        Json(Value::Object(items)),
    ))
}
