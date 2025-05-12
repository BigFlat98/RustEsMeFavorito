use sqlx::mysql::MySqlPool;
use std::env;
use dotenvy::dotenv;

pub async fn say_hello() -> Result<(), sqlx::Error> {
    dotenv().ok();
    
    // 데이터베이스 연결 정보
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // 데이터베이스 연결
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");
    
    println!("데이터베이스 연결 성공!");
    
    // 연결 테스트를 위한 간단한 쿼리 실행
    let result = sqlx::query("SELECT 1")
        .execute(&pool)
        .await?;
    
    println!("쿼리 실행 결과: {:?}", result);
    
    Ok(())
} 