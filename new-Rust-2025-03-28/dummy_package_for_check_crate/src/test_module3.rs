use mysql::*;
use mysql::prelude::*;

pub fn say_hello() {
    // MySQL 연결 설정
    let url = "mysql://username:password@localhost:3306/database_name";
    
    match Pool::new(url) {
        Ok(pool) => {
            println!("MySQL 데이터베이스에 성공적으로 연결되었습니다!");
            
            // 연결 풀에서 연결 가져오기
            match pool.get_conn() {
                Ok(mut conn) => {
                    // 간단한 쿼리 실행 예시
                    match conn.query_first::<String>("SELECT VERSION()") {
                        Ok(Some(version)) => println!("MySQL 버전: {}", version),
                        Ok(None) => println!("버전 정보를 가져올 수 없습니다."),
                        Err(e) => println!("쿼리 실행 중 오류 발생: {}", e),
                    }
                },
                Err(e) => println!("데이터베이스 연결 중 오류 발생: {}", e),
            }
        },
        Err(e) => println!("데이터베이스 연결 풀 생성 중 오류 발생: {}", e),
    }
} 