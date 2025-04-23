use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

// 게시글 구조체
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Post {
    id: i32,
    title: String,
    content: String,
}

// API 상태
#[derive(Clone)]
struct AppState {
    posts: Arc<RwLock<Vec<Post>>>,
}

// 게시글 생성 요청
#[derive(Debug, Deserialize)]
struct CreatePost {
    title: String,
    content: String,
}

// 게시글 수정 요청
#[derive(Debug, Deserialize)]
struct UpdatePost {
    title: Option<String>,
    content: Option<String>,
}

// 게시글 목록 조회
async fn get_posts(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let posts = state.posts.read().await;
    Json(posts.clone())
}

// 게시글 상세 조회
async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Post>, StatusCode> {
    let posts = state.posts.read().await;
    let post = posts.iter().find(|p| p.id == id);
    
    match post {
        Some(post) => Ok(Json(post.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// 게시글 생성
async fn create_post(
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> impl IntoResponse {
    let mut posts = state.posts.write().await;
    let id = posts.len() as i32 + 1;
    
    let post = Post {
        id,
        title: payload.title,
        content: payload.content,
    };
    
    posts.push(post.clone());
    (StatusCode::CREATED, Json(post))
}

// 게시글 수정
async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode> {
    let mut posts = state.posts.write().await;
    let post = posts.iter_mut().find(|p| p.id == id);
    
    match post {
        Some(post) => {
            if let Some(title) = payload.title {
                post.title = title;
            }
            if let Some(content) = payload.content {
                post.content = content;
            }
            Ok(Json(post.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// 게시글 삭제
async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    let mut posts = state.posts.write().await;
    let index = posts.iter().position(|p| p.id == id);
    
    match index {
        Some(index) => {
            posts.remove(index);
            StatusCode::NO_CONTENT
        }
        None => StatusCode::NOT_FOUND,
    }
}

#[tokio::main]
async fn main() {
    // 초기 데이터
    let posts = vec![
        Post {
            id: 1,
            title: "첫 번째 게시글".to_string(),
            content: "안녕하세요!".to_string(),
        }
    ];

    let state = AppState {
        posts: Arc::new(RwLock::new(posts)),
    };

    let app = Router::new()
        .route("/posts", get(get_posts))
        .route("/posts", post(create_post))
        .route("/posts/:id", get(get_post))
        .route("/posts/:id", put(update_post))
        .route("/posts/:id", delete(delete_post))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("서버 시작: http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
