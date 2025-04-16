use axum::{
    Router,
    middleware::{self, Next},
    http::{Request, Response, StatusCode},
    routing::{get, post},
    body::Body,
    response::IntoResponse,
    extract::{State,Extension}, //상태 공유
};
use std::sync::Arc;
use tokio::sync::RwLock;

//공유할 상태 구조체
#[derive(Clone)]
struct AppState{
    counter: Arc<RwLock<i32>>,//스레드 안전한 카운터. vue에서 state의 값을 읽거나 수정할 때 mutation, action을 사용하듯이 RwLock을 통해 안전히 수정한다.
            // RwLock<i32>은 다중 스레드 처리가 가능한 구조체
            //DB에 연동하거나 캐시를 활용하는 방법도 있음. 
}

//상태 공유는 여러 핸들러나 미들웨어에서 공통으로 사용할 데이터를 관리하는 방법. vue의 store와 유사한 개념으로 사용되는 듯 함
//상태를 사용하는 핸들러
async fn get_counter(
    State(state): State<AppState>
)->String{
    let counter = state.counter.read().await;
    format!("현재 카운터:{}",*counter)
}

//카운터 증가 핸들러
async fn increment_counter(
    State(state): State<AppState>
)->String{
    let mut counter = state.counter.write().await;
    *counter +=1;
    format!("카운터 증가:{}",*counter)
}

//상태를 사용하는 미들웨어
async fn counter_middleware(
    Extension(state): Extension<AppState>,
    req: Request<Body>,
    next: Next,
)->Response<Body>{
    println!("현재 카운터: {}",*state.counter.read().await);
    next.run(req).await
}



//인증 미들웨어
async fn auth_middleware(
    req: Request<Body>,
    next:Next,
)->Result<Response<Body>,(StatusCode,&'static str)>{
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    match auth_header {
        Some(token) if token == "secret" =>{
            //인증 성공 시 다음 미들웨어/핸들러 실행
            Ok(next.run(req).await)
        }
        _=>{
            //인증 실패 시 401 에러 반환
            Err((StatusCode::UNAUTHORIZED,"need authorization"))
        }
    }
}

// 에러 처리 미들웨어
async fn error_middleware(
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    // 다음 미들웨어/핸들러 실행
    let response = next.run(req).await;
    
    // 응답 상태 코드 확인
    if response.status().is_server_error() {
        eprintln!("서버 에러 발생: {}", response.status());
    }
    
    response
}

// 로깅 미들웨어
async fn logging_middleware(
    req: Request<Body>,
    next: Next,
) -> Response<Body> {
    println!("요청: {} {}", req.method(), req.uri());
    let response = next.run(req).await;
    println!("응답: {}", response.status());
    response
}

#[tokio::main]
async fn main() {
    let state = AppState{
        counter: Arc::new(RwLock::new(0)),
    };
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/error", get(error_handler))
        .route("/protected",get(protected_handler))
        .route("/counter",get(get_counter))
        .route("/increment",get(increment_counter))
        .layer(Extension(counter_middleware))
        .with_state(state)
        .layer(middleware::from_fn(error_middleware))
        .layer(middleware::from_fn(auth_middleware))
        .layer(middleware::from_fn(logging_middleware));
    //**중요! : 미들웨어가 처리되는 순서는 layer 메서드가 적힌 "역순"으로 실행된다.
    // 실제로는 이렇게 중첩됨
    // logging_middleware(
    //     auth_middleware(
    //         router
    //     )
    // )

    //loggin_middleware와 auth_middleware만 있다고 가정했을 때.
    // 1. 요청 들어옴
    // 2. logging_middleware 실행
    //    - "로그 시작" 출력
    //    - next.run() 호출 (auth_middleware로 이동)
    // 3. auth_middleware 실행
    //    - "인증 시작" 출력
    //    - next.run() 호출 (router로 이동)
    // 4. router 실행
    // 5. auth_middleware 마무리
    //    - "인증 끝" 출력
    // 6. logging_middleware 마무리
    //    - "로그 끝" 출력
    // 7. 응답 반환

    //layer와 router의 순서는 섞여도 상관없음. 내부적으로 layer -> route 실행순서를 갖기 때문에. layer끼리의 순서만 잘 적어주면 됨.

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("서버 시작: http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

// 기본 핸들러
async fn hello_world() -> &'static str {
    "Hello, World!"
}

// 에러 발생 테스트용 핸들러
async fn error_handler() -> (StatusCode, &'static str) {
    (StatusCode::INTERNAL_SERVER_ERROR, "테스트 에러 발생!")
}

async fn protected_handler() -> &'static str{
    "access the resource"
}



