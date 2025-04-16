use axum::{
    Router,
    routing::{get,post},
    extract::{Path,Query,Json},
    http::StatusCode,
    response::IntoResponse,
};

//경로 매개변수 예시
async fn get_user(Path(id): Path<i32>) -> impl IntoResponse{
    format!("user id : {}", id)
}

//쿼리 매개변수 예시
#[derive(serde::Deserialize)]
struct SearchParams{
    q: String,
    page: Option<i32>,
}

async fn search_users(Query(params): Query<SearchParams>)->impl IntoResponse{
    format!("검색어: {}, 페이지: {:?}", params.q, params.page)
}

//중첩 라우팅 예시
fn api_routes() -> Router{
    Router::new()
        .route("/users",get(get_users))
        .route("/users/:id",get(get_user))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        //기본 라우터
        .route("/",get(hello_world))

        //경로 매개변수
        .route("/users/:id",get(get_user))

        //쿼리 매개변수
        .route("/search",get(search_users))

        //중첩 라우팅
        .nest("/api",api_routes());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("catch error when server starting");//bind() -> 지정된 주소와 포트에 서버 바인딩
        println!("server start on http://127.0.0.1:3000");

        //서버 실행
        axum::serve(listener,app).await.expect("");
}

async fn get_users()->&'static str{
    "user list"
}
async fn hello_world()->&'static str{
    "hello world"
}
