use std::{
    env,
    net::SocketAddr,
    sync::{
        atomic::{AtomicI64, Ordering}, Arc,
        Mutex,
    },
};

use axum::{
    extract::State, http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json,
    Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    instance_id: Uuid,
    users: Arc<Mutex<Vec<NevenaUser>>>,
    next_user_id: Arc<AtomicI64>,
}

const VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
struct NevenaUser {
    id: i64,
    email: String,

    #[serde(serialize_with = "serialize_datetime")]
    time: DateTime<Utc>,
}

#[derive(Serialize)]
struct UsersResponse {
    users: Vec<NevenaUser>,
    version: &'static str,
    count: usize,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
    instance_id: Uuid,

    #[serde(serialize_with = "serialize_datetime")]
    time: DateTime<Utc>,
    version: &'static str,
}

#[derive(Debug, Serialize)]
struct DeleteResponse {
    version: &'static str,
    count: usize,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    code: &'static str,
    message: &'static str,
}

#[derive(Debug)]
enum AppError {
    NotFound,
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Self::NotFound => "not_found",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Self::NotFound => "Route not found",
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        let body = Json(ErrorResponse {
            error: ErrorBody {
                code: self.code(),
                message: self.message(),
            },
        });

        (status, body).into_response()
    }
}

fn serialize_datetime<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let formatted = dt.format("%Y-%m-%dT%H:%M:%S").to_string();
    serializer.serialize_str(&formatted)
}

fn make_user(id: i64, email: String) -> NevenaUser {
    NevenaUser {
        id,
        email,
        time: Utc::now(),
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CreateUserRequest {}

async fn create_user_handler(
    State(state): State<AppState>,
    Json(_): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let id = state.next_user_id.fetch_add(1, Ordering::SeqCst);
    let email = format!("igor_{}@example.com", id);
    let user = make_user(id, email);
    state.users.lock().unwrap().push(user.clone());
    (StatusCode::CREATED, Json(user))
}

async fn delete_users_handler(State(state): State<AppState>) -> impl IntoResponse {
    let count = {
        let mut users = state.users.lock().unwrap();
        let len = users.len();
        users.clear();
        len
    };
    state.next_user_id.store(1, Ordering::SeqCst);
    Json(DeleteResponse {
        version: VERSION,
        count,
    })
}

async fn user_handler(State(state): State<AppState>) -> Json<UsersResponse> {
    debug!("Handling request for /");
    let users = state.users.lock().unwrap();
    Json(UsersResponse {
        count: users.len(),
        users: users.clone(),
        version: VERSION,
    })
}

async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    debug!("Handling request for /healthz");
    Json(HealthResponse {
        status: "ok",
        instance_id: state.instance_id,
        time: Utc::now(),
        version: VERSION,
    })
}

async fn not_found() -> AppError {
    debug!("Handling unknown route");
    AppError::NotFound
}

fn build_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/",
            get(user_handler)
                .post(create_user_handler)
                .delete(delete_users_handler),
        )
        .route("/healthz", get(health_handler))
        .fallback(not_found)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

fn read_port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(8000)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug,tower_http=debug,axum=debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let instance_id = Uuid::new_v4();
    let users = Arc::new(Mutex::new(vec![]));
    let next_user_id = Arc::new(AtomicI64::new(1));
    let state = AppState {
        instance_id,
        users,
        next_user_id,
    };

    let id = 1;
    let email = format!("igor_{}@example.com", id);
    let sample_payload = serde_json::to_string_pretty(&make_user(id, email))?;
    let port = read_port();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let app = build_router(state);

    info!(
        %addr,
        %instance_id,
        sample_payload = %sample_payload,
        "starting server"
    );

    let listener = tokio::net::TcpListener::bind(addr).await.map_err(|err| {
        error!(%addr, error = %err, "failed to bind TCP listener");
        err
    })?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .map_err(|err| {
            error!(error = %err, "server terminated with error");
            err
        })?;

    Ok(())
}
