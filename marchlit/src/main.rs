use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::State, http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json,
    Router,
};
use tower_http::services::ServeDir;

const PORT_ENV: &str = "PORT";
const DEFAULT_PORT: u16 = 8000;

#[derive(Clone)]
struct AppState {
    index_html: Arc<str>,
    articles: Arc<std::sync::Mutex<Vec<Article>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(default = "uuid::Uuid::new_v4")]
    id: uuid::Uuid,
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateUserRequest {
    user: User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Article {
    #[serde(default = "uuid::Uuid::new_v4")]
    id: uuid::Uuid,
    author_id: uuid::Uuid,
    title: String,
    description: String,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateArticleRequest {
    article: Article,
}

#[derive(Debug, Serialize, Deserialize)]
struct ArticleListItem {
    id: uuid::Uuid,
    author_id: uuid::Uuid,
    title: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetArticlesResponse {
    articles: Vec<ArticleListItem>,
    #[serde(rename = "articlesCount")]
    articles_count: usize,
}

#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
enum AppError {
    #[error("internal server error")]
    Internal,
    #[error("validation error: {0}")]
    ValidationError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Internal => {
                let body = Json(serde_json::json!({
                    "error": "internal server error"
                }));
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
            AppError::ValidationError(msg) => {
                let body = Json(serde_json::json!({
                    "error": msg
                }));
                (StatusCode::UNPROCESSABLE_ENTITY, body).into_response()
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let port = std::env::var(PORT_ENV)
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    let state = AppState {
        index_html: Arc::from(std::fs::read_to_string("static/index.html")?),
        articles: Arc::new(std::sync::Mutex::new(vec![Article {
            id: uuid::Uuid::new_v4(),
            author_id: uuid::Uuid::new_v4(),
            title: "First Article".to_string(),
            description: "First description".to_string(),
            body: "First body".to_string(),
        }])),
    };

    let app = Router::new()
        .route("/", get(get_page_index))
        .route("/api/users", post(post_users))
        .route(
            "/api/articles",
            post(post_articles)
                .get(get_articles)
                .delete(delete_articles),
        )
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let printable_uuid = uuid::Uuid::new_v4();
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    log::info!("Starting server on http://{addr} [{printable_uuid}]");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn get_page_index(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    log::debug!("get_page_index");
    Ok(Html(state.index_html.to_string()))
}

async fn post_users(
    State(_state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("post_users: {:?}", payload);
    Ok((StatusCode::CREATED, Json(payload)))
}

async fn post_articles(
    State(state): State<AppState>,
    Json(payload): Json<CreateArticleRequest>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("post_articles: {:?}", payload);
    if payload.article.author_id.is_nil() {
        return Err(AppError::ValidationError(
            "author_id must not be nil".to_string(),
        ));
    }
    state.articles.lock().unwrap().push(payload.article.clone());
    Ok((StatusCode::CREATED, Json(payload)))
}

async fn get_articles(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    log::debug!("get_articles");
    let articles = state.articles.lock().unwrap();
    let articles_list: Vec<ArticleListItem> = articles
        .iter()
        .map(|a| ArticleListItem {
            id: a.id,
            author_id: a.author_id,
            title: a.title.clone(),
            description: a.description.clone(),
        })
        .collect();

    let response = GetArticlesResponse {
        articles_count: articles_list.len(),
        articles: articles_list,
    };

    Ok(Json(response))
}

async fn delete_articles(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    log::debug!("delete_articles");
    state.articles.lock().unwrap().clear();
    Ok(StatusCode::NO_CONTENT)
}
