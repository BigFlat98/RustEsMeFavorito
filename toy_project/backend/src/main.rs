use axum::{
    Router,
    routing::{get,post,put,delete},
    extract::{Json,Path,State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::mysql::MySqlPool;
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;


//api state. 어플리
#[derive(Clone)] //여러 라우트 핸들러에게 상태 공유를 위한 어노테이션
struct AppState{
    db:MySqlPool,
} //다른 이벤트 핸들러에서 DB사용을 위해 필요.

//post struct
//Serialize,Deserialize -> json 데이터를 구조체로 변조, 복조 하기 위한 어노테이션
#[derive(Debug,Serialize,Deserialize)] //debug -> println!("{:?}", post) 또는 println!("{:#?}", post)와 같은 형식으로 구조체의 내용을 출력할 수 있습니다. / ? 연산자를 사용할 때 에러 정보를 출력하는데 사용됩니다.
struct Post{
    id:i64,
    title: Option<String>, //null이 올 수 있는 모든 속성은 Option 타입을 사용해줘야 컴파일됨. -> 이 개같은 것 때문에 1시간 넘게 씀
    content: Option<String>,
    writer: Option<String>,
}

//get all post
async fn get_all_posts(
    State(state):State<AppState>,
) -> Result<Json<Vec<Post>>,StatusCode>{
    let posts = sqlx::query_as!(
        Post,
        "SELECT p.id as id, p.title as title, p.content as content, u.name as writer FROM post as p LEFT JOIN user as u ON p.user_id = u.id ORDER BY p.id"
    )
    .fetch_all(&state.db) //db에서 모든 결과를 가져오는 메서드. 쿼리 결과를 Vec<Post>로 변환
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(posts))
}

//get specifit post
async fn get_post(
    State(state):State<AppState>,
    Path(id):Path<i32>
)-> Result<Json<Post>,StatusCode>{
    let post = sqlx::query_as!(
        Post,
        "select p.id, p.title, p.content, u.name as writer from post as p left join user as u on p.user_id = u.id where p.id = ?",
        id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;//ok_or -> option 타입을 result타입으로 변환
    //Some(value) → Ok(value)
    //None → Err(StatusCode::NOT_FOUND)
    
    Ok(Json(post))
}


#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    
    //db connect
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be exist");
    let db = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    let state = AppState{db}; //vue의 store 같은 기능.

    let app = Router::new()
        .route("/get_all_posts",get(get_all_posts))
        .route("/get_post/:id",get(get_post))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",port)).await.expect("Failed to bind to port");
    println!("Server is running on http://127.0.0.1:{}",port);
    axum::serve(listener, app.into_make_service()).await.expect("Failed to serve");
}
