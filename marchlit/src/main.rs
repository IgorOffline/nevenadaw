use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicU64, Ordering}, Arc,
        Mutex,
    },
};

use axum::{
    extract::{Path, State}, http::StatusCode,
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
    articles: Arc<Mutex<Vec<Article>>>,
    comments: Arc<Mutex<Vec<(uuid::Uuid, Comment)>>>,
    next_comment_id: Arc<AtomicU64>,
    last_uid: Arc<Mutex<String>>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Comment {
    id: u64,
    body: String,
    author: CommentAuthor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct CommentAuthor {
    username: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateCommentRequest {
    comment: CreateCommentInner,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateCommentInner {
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommentResponse {
    comment: Comment,
}

#[derive(Debug, Serialize, Deserialize)]
struct GetCommentsResponse {
    comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FavoriteArticleResponse {
    article: FavoriteArticleInner,
}

#[derive(Debug, Serialize, Deserialize)]
struct FavoriteArticleInner {
    title: String,
    description: String,
    body: String,
    favorited: bool,
    #[serde(rename = "favoritesCount")]
    favorites_count: u64,
    author: FavoriteArticleAuthor,
}

#[derive(Debug, Serialize, Deserialize)]
struct FavoriteArticleAuthor {
    username: String,
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
        articles: Arc::new(Mutex::new(vec![Article {
            id: uuid::Uuid::new_v4(),
            author_id: uuid::Uuid::new_v4(),
            title: "First Article".to_string(),
            description: "First description".to_string(),
            body: "First body".to_string(),
        }])),
        comments: Arc::new(Mutex::new(Vec::new())),
        next_comment_id: Arc::new(AtomicU64::new(1)),
        last_uid: Arc::new(Mutex::new("user".to_string())),
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
        .route(
            "/api/articles/{article_id}/comments",
            post(post_comments)
                .get(get_comments)
                .delete(delete_comments),
        )
        .route("/api/articles/{article_id}/favorite", post(post_favorite))
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
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("post_users: {:?}", payload);

    let username = &payload.user.username;
    if let Some(uid) = username.strip_prefix("art_") {
        *state.last_uid.lock().unwrap() = uid.to_string();
    } else {
        *state.last_uid.lock().unwrap() = username.clone();
    }

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

async fn post_comments(
    State(state): State<AppState>,
    Path(article_id): Path<uuid::Uuid>,
    Json(payload): Json<CreateCommentRequest>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("post_comments for article {}: {:?}", article_id, payload);

    // Mocking comment creation
    let id = state.next_comment_id.fetch_add(1, Ordering::SeqCst);
    let uid = state.last_uid.lock().unwrap().clone();
    let comment = Comment {
        id,
        body: payload.comment.body,
        author: CommentAuthor {
            username: format!("cmt_{}", uid),
        },
    };

    state
        .comments
        .lock()
        .unwrap()
        .push((article_id, comment.clone()));

    Ok((StatusCode::CREATED, Json(CommentResponse { comment })))
}

async fn get_comments(
    State(state): State<AppState>,
    Path(article_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("get_comments for article {}", article_id);

    let comments = state.comments.lock().unwrap();
    let article_comments: Vec<Comment> = comments
        .iter()
        .filter(|(id, _)| *id == article_id)
        .map(|(_, c)| c.clone())
        .collect();

    Ok(Json(GetCommentsResponse {
        comments: article_comments,
    }))
}

async fn delete_articles(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    log::debug!("delete_articles");
    state.articles.lock().unwrap().clear();
    state.comments.lock().unwrap().clear();
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_comments(
    State(state): State<AppState>,
    Path(article_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("delete_comments for article {}", article_id);
    state
        .comments
        .lock()
        .unwrap()
        .retain(|(id, _)| *id != article_id);
    Ok(StatusCode::NO_CONTENT)
}

async fn post_favorite(
    State(state): State<AppState>,
    Path(article_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, AppError> {
    log::debug!("post_favorite for article {}", article_id);

    let articles = state.articles.lock().unwrap();
    let article = articles
        .iter()
        .find(|a| a.id == article_id)
        .cloned()
        .ok_or(AppError::Internal)?;

    let uid = state.last_uid.lock().unwrap().clone();

    let response = FavoriteArticleResponse {
        article: FavoriteArticleInner {
            title: article.title,
            description: article.description,
            body: article.body,
            favorited: true,
            favorites_count: 1,
            author: FavoriteArticleAuthor {
                username: format!("fav_{}", uid),
            },
        },
    };

    Ok(Json(response))
}
