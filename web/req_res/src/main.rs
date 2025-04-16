use axum::{
    Router,
    routing::{get,post},
    extract::Json,
    http::{Request,Response,StatusCode, HeaderMap, HeaderValue},
    response::{IntoResponse,Response as AxumResponse},
    body::Body,
};
use headers::{UserAgent,HeaderMapExt};
use axum_extra::TypedHeader;

//요청 헤더 처리
async fn header_handler(
    headers: HeaderMap, //요청의 모든 헤더를 받음
) -> impl IntoResponse {
    //User-Agent 헤더 추출
    let user_agent = headers
        .get("user-agent") //헤더 추출
        .and_then(|h| h.to_str().ok()) //문자열로 변환, Option의 메서드 and_then. some
        .unwrap_or("unknown"); //없으면 unknown 반환
    
    format!("User-Agent: {}", user_agent)
}

// 커스텀 응답 생성
async fn custom_response() -> impl IntoResponse{
    //응답 생성
    let mut response: AxumResponse = Response::new("Hello, world".into());

    //헤더 추가
    response.headers_mut().insert(
        "X-Custom-Header",
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );

    //상태 코드 설정
    *response.status_mut() = StatusCode::CREATED;

    response
}

//Json 요청/응답
#[derive(serde::Deserialize,serde::Serialize)]
struct User{
    name:String,
    email:String,
}

async fn create_user(
    Json(user): Json<User>, //json 본문을 user 구조체로 변환
) -> impl IntoResponse{
    //사용자 생성
    let response = Json(User {
        name: user.name,
        email: user.email,
    });

    (StatusCode::CREATED,response) //201 상태코드와 함께 응답
}
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/header",get(header_handler))
        .route("/custom",get(custom_response))
        .route("/users",post(create_user));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("catch error when server starting");
    println!("server start on http://127.0.0.1:3000");

    axum::serve(listener,app.into_make_service()).await.expect("catch error when server starting");
}
