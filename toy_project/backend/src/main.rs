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

//add post struct
#[derive(Debug,Deserialize)] //Json을 Struct로 변환하는게 Deserialize
struct AddPost{
    title: String,
    content: String,
    user_id: i64,
}

//information for deleting post
#[derive(Debug, Deserialize)]
struct DeleteInfo{
    user_id: i64,
    post_id: i64,
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
    .fetch_optional(&state.db) //결과를 option 타입으로 반환
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;//ok_or -> option 타입을 result타입으로 변환
    //Some(value) → Ok(value)
    //None → Err(StatusCode::NOT_FOUND)
    
    Ok(Json(post))
}


//add post
async fn add_post(
    State(state):State<AppState>,
    Json(payload):Json<AddPost>, //json으로 프론트에서 받은 데이터 AddPost 타입으로 Deserialize. 해당 값은 payload로 참조
) -> Result<impl IntoResponse,StatusCode>{

    let find_user = sqlx::query!(
        "select id, name from user where id = ?",
        payload.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if find_user.is_none(){
        return Err(StatusCode::NOT_FOUND); //not_found 에러는 404를 반환함
    }

    let result = sqlx::query!(
        "insert into post (title, content, user_id) values (?, ?, ?)",
        payload.title,
        payload.content,
        payload.user_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let addedpost = sqlx::query_as!(
        Post,
        "select p.id, p.title, p.content, u.name as writer from post as p left join user as u on p.user_id = u.id where p.id = ?",
        result.last_insert_id() as i64
    )
    .fetch_one(&state.db) //결과를 Result 타입으로 반환. 값을 검색해서 가져올 때, 값이 없을 수도 있기 때문에 사용
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED,Json(addedpost)))
}


//update post -> 지금 수정 요청을 하는 사용자 검사를 안함.
async fn update_post(
    State(state):State<AppState>,
    Path(post_id):Path<i64>,
    Json(payload):Json<AddPost>,
) -> Result<impl IntoResponse, StatusCode>{
    let find_post = sqlx::query!(
        "select id, user_id from post where id = ?",
        post_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if find_post.is_none(){
        return Err(StatusCode::NOT_FOUND);
    }

    if find_post.unwrap().user_id != payload.user_id{
        return Err(StatusCode::FORBIDDEN); //403 에러 반환
    }

    let result = sqlx::query!(
        "update post set title = ?, content = ? where id = ?",
        payload.title,
        payload.content,
        post_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let updated_post = sqlx::query_as!(
        Post,
        "select p.id, p.title, p.content, u.name as writer from post as p left join user as u on p.user_id = u.id where p.id = ?",
        post_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK,Json(updated_post)))
}


//delete post
async fn delete_post(
    State(state):State<AppState>,
    Json(payload):Json<DeleteInfo>,
)-> Result<StatusCode, StatusCode>{
    let find_post = sqlx::query!(
        "select * from post where id = ? and user_id = ?", //post id 와 user id 를 동시에 갖는 post를 찾는 방식 -> user 가 다르면 결과가 안나와 404 전달. 이게 조건문 쓰는 것 보다 좋은 것 같음.
        payload.post_id,
        payload.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    if find_post.is_none(){
        return Err(StatusCode::NOT_FOUND);
    }

    let result = sqlx::query!(
        "delete from post where id = ? and user_id = ?",
        payload.post_id,
        payload.user_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(StatusCode::OK)
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
        .route("/add_post",post(add_post))
        .route("/update_post/:post_id",put(update_post))
        .route("/delete_post",delete(delete_post))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}",port)).await.expect("Failed to bind to port");
    println!("Server is running on http://127.0.0.1:{}",port);
    axum::serve(listener, app.into_make_service()).await.expect("Failed to serve");
}
