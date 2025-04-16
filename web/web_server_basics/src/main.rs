use axum::{ //프레임워크 모듈 import
    routing::{get,post,put,delete,patch}, //http get요청을 처리하기 위한 라우팅 핸들러
    Json, //Axum의 json 처리기
    Router, //요청을 적절한 핸들러로 라우팅하는 핵심 컴포넌트
    http::StatusCode,
    response::{IntoResponse,Response},
};
use serde::{Deserialize,Serialize}; //serde의 직렬화/역직렬화 트레잇




//에러 타입 정의
#[derive(Debug)]
enum AppError{
    UserNotFound,
    InvalidData,
}
impl IntoResponse for AppError{ //IntoResponse를 사용하는 이유 -> 우리가 만든 이벤트 핸들러에서는 꼭 응답이 필요한데, 매번 만들지 말고 IntoResponse를 구현해 .into_response를 사용해서 다양한 값을 응답할 수 있도록 한 것. 응답을 여러면 반복적으로 만드는 일을 없애기 위해서 존재하는 것 같음
    fn into_response(self) -> Response{
        //에러의 타입에 따라 상태 코드와 메시지 결정
        let (status, message) = match self{ //튜플을 이용한 매칭 패턴
            AppError::UserNotFound => (StatusCode::NOT_FOUND, "can not find user"),
            AppError::InvalidData => (StatusCode::BAD_REQUEST, "Wrong user request"),
        }; // -> 이 결과로 status에는 StatusCode의 결과가, Message에는 "can not find user" 나 "Wrong user request" 문자열이 들어감
        //(상태,메시지) 튜플을 Http 응답으로 변환
        (status,message).into_response() //-> IntoResponse::into_response((status,message)) 와 같은 로직


        //**설명**
        //	•	나는 MyError 타입에 대해 into_response()를 구현한 거고,
	    //  •	그 구현 안에서 (StatusCode, &str) 라는 전혀 다른 타입의 .into_response()를 호출하고 있는 것
        //즉, 네가 새로 정의한 건 AppError의 변환 방식이고,
        //그 안에서 Axum이 이미 제공하는 (StatusCode, &str) → Response 변환 기능을 이용한 거지.

        //결론
        // •	네가 정의한 into_response()는 MyError를 어떤 응답으로 바꿀지 “직접” 정하는 곳이 맞아.
        // •	그 안에서 또 .into_response()를 호출하는 건, Axum이 이미 제공하는 응답 생성 기능을 가져다 쓴 것일 뿐.
    }
}


//API 응답 타입
#[derive(Serialize)]
struct ApiResponse<T>{
    success:bool,
    data: Option<T>,
    message: Option<String>,
}



//사용자 데이터 구조체
#[derive(Serialize,Deserialize)]//json 변환을 위한 매크로
struct User{
    id:u32,
    name: String,
    email: String,
}

#[tokio::main] //스프링의 어노테이션과 유사한 기능. 비동기 런타임을 위한 매크로
async fn main() {

    //기본 라우터 생성
    let app = Router::new() //새로운 라우터 인스턴스 생성
                .route("/",get(hello_world)) //경로와 핸들러 함수 연결
                .route("/user",get(get_users))
                .route("/create_user",post(create_user))
                .route("/user/:id",get(get_user_by_id));

    //서버 설정
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.expect("catch error when server starting");//bind() -> 지정된 주소와 포트에 서버 바인딩
    println!("server start on http://127.0.0.1:3000");

    //서버 실행
    axum::serve(listener,app).await.expect("");

}

//요청 핸들러. 실제 요청을 처리하는 함수
async fn hello_world() -> &'static str{ //async는 비동기 처리를 동기 코드처럼 작동 키워드. 다양한 반환 타입 가능(문자열, json, 상태코드 등)
    "Hello,world!"
}

//성공 응답 헬퍼 메서드
fn success_response<T: Serialize>(data: T) -> Json<ApiResponse<T>>{
    Json(ApiResponse{
        success: true,
        data:Some(data),
        message:None,
    })
}

async fn get_users() -> impl IntoResponse {
    let users = vec![
        User{
            id:1,
            name:String::from("hhh"),
            email:String::from("hhh@naver.com"),
        },
        User{
            id:2,
            name: String::from("ttt"),
            email:String::from("ttt@naver.com"),
        },
    ];
    success_response(users) //아래 적어놓은 설명으로 인해 Json타입을 반환하는 success_response가 이 메서드의 반환형에 맞음.

    // axum 내부에서 Json 타입은 IntoResponse를 구현합니다
    // impl<T: Serialize> IntoResponse for Json<T> {
    //     fn into_response(self) -> Response {
    //         // JSON 응답 생성 로직
    //     }
    // }
    
}

async fn create_user(Json(user): Json<User>) -> Result<impl IntoResponse, AppError> {//결과는 Result 타입이고 Ok의 경우 그 결과가 어떤 타입이건 IntoResponse를 구현하는 타입이어야 함, Err는 결과가 AppError여야 함. impl의 독특한 사용법임.

    //간단한 유효성 검사
    if user.name.is_empty() || user.email.is_empty(){
        return Err(AppError::InvalidData);
    }
    Ok((StatusCode::CREATED, success_response(user)))
}

async fn get_user_by_id(axum::extract::Path(id): axum::extract::Path<u32>) -> Result<impl IntoResponse,AppError>{
    //1인 사용자만 반환
    if id == 1 {
        let user = User{
            id:1,
            name:String::from("hhh"),
            email:String::from("hhh@naver.com"),
        };
        Ok(success_response(user))
    }
    else{
        Err(AppError::UserNotFound)
    }
    

}