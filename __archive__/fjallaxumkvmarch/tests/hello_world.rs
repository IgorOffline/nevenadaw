use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    routing::get,
    Router,
};
use fjall::{Database, KeyspaceCreateOptions, PersistMode};
use http_body_util::BodyExt;
use log::info;
use serde_json::json;
use tempfile::tempdir;
use thiserror::Error;
use tower::ServiceExt;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Database error")]
    DbError,
}

#[tokio::test]
async fn test_hello_world_all_deps() {
    let _ = env_logger::builder().is_test(true).try_init();
    info!("Starting 'Hello, World!' test with all dependencies");

    let dir = tempdir().expect("Failed to create temp dir");
    let db = Database::builder(dir.path())
        .open()
        .expect("Failed to open database");
    let partition = db
        .keyspace("hello", KeyspaceCreateOptions::default)
        .expect("Failed to open keyspace");

    partition
        .insert("greeting", "Hello, World!")
        .expect("Failed to insert");
    db.persist(PersistMode::SyncAll).expect("Failed to persist");

    let stored_value_opt = partition.get("greeting").expect("Failed to get");
    let stored_value = stored_value_opt.expect("Value not found");

    let message = String::from_utf8(stored_value.to_vec()).expect("Invalid UTF-8");

    let mut ser = serde_json::Serializer::new(Vec::new());
    serde::Serialize::serialize(&message, &mut ser).expect("Failed to use serde");

    let json_data = json!({ "message": message }).to_string();

    let app = Router::new().route(
        "/",
        get(move || async move {
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())],
                json_data,
            )
        }),
    );

    let request = Request::builder()
        .uri("/")
        .body(Body::empty())
        .expect("Failed to build request");

    let response = app
        .oneshot(request)
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = response
        .into_body()
        .collect()
        .await
        .expect("Failed to collect body")
        .to_bytes();
    let final_data: serde_json::Value =
        serde_json::from_slice(&body_bytes).expect("Failed to deserialize JSON");

    assert_eq!(final_data["message"], "Hello, World!");

    let err = MyError::DbError;
    info!("Encountered expected error for testing: {}", err);

    info!("Test finished successfully: {}", final_data["message"]);
}
