use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPool;
use dotenvy::dotenv;
use std::env;




// 게시글 구조체
#[derive(Debug, Serialize, Deserialize)]
struct Post {
    id: i32,
    title: String,
    content: String,
}

//api 상태
#[derive(Clone)]
struct AppState{
    db:MySqlPool,
}

#[derive(Debug, Deserialize)]
struct CreatePost {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct UpdatePost {
    title: Option<String>, //값이 없으면 None 저장
    content: Option<String>,
}

//게시물 목록 조회
async fn get_posts(
    State(state):State<AppState>,
) -> Result<Json<Vec<Post>>,StatusCode>{
    let posts = sqlx::query_as!(
        Post,
        "SELECT id, title, content FROM posts ORDER BY id"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

//게시글 상세 조회
async fn get_post(
    State(state):State<AppState>,
    Path(id):Path<i32>,

) -> Result<Json<Post>,StatusCode>{
    let post = sqlx::query_as!(
        Post,
        "SELECT id, title, content FROM posts WHERE id = ?",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(post))
}

//게시글 생성
async fn create_post(
    State(state):State<AppState>,
    Json(payload):Json<CreatePost>,
    ) -> Result<impl IntoResponse,StatusCode>{
        let result = sqlx::query!(
            "INSERT INTO posts (title, content) VALUES (?, ?)",
            payload.title,
            payload.content
        )
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let post = sqlx::query_as!(
        Post,
        "SELECT id, title, content FROM posts WHERE id = ?",
        result.last_insert_id() as i32
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED,Json(post)))
}

// 게시글 수정
async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePost>,
) -> Result<Json<Post>, StatusCode> {
    // 먼저 게시글 존재 여부 확인
    let post = sqlx::query_as!(
        Post,
        "SELECT id, title, content FROM posts WHERE id = ?",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // 업데이트 실행
    sqlx::query!(
        "UPDATE posts SET title = COALESCE(?, title), content = COALESCE(?, content) WHERE id = ?",
        payload.title,
        payload.content,
        id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 업데이트된 게시글 조회
    let updated_post = sqlx::query_as!(
        Post,
        "SELECT id, title, content FROM posts WHERE id = ?",
        id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_post))
}

// 게시글 삭제
async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> StatusCode {
    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = ?",
        id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                StatusCode::NO_CONTENT
            } else {
                StatusCode::NOT_FOUND
            }
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

//처음 필요한 테이블 생성
async fn create_table(db: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INT AUTO_INCREMENT PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(db)
    .await?;

    println!("테이블 생성 완료");
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();    
    //db연결
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    println!("port: {}", port);
    let db = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    // 테이블 생성
    create_table(&db)
        .await
        .expect("Failed to create table");

    let state = AppState{ db };
    let app = Router::new()
        .route("/posts", get(get_posts))
        .route("/posts", post(create_post))
        .route("/posts/:id", get(get_post))
        .route("/posts/:id", put(update_post))
        .route("/posts/:id", delete(delete_post))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await.expect("Failed to bind to port");
    println!("Server is running on http://127.0.0.1:3002");
    axum::serve(listener, app.into_make_service()).await.expect("Failed to serve");
}
