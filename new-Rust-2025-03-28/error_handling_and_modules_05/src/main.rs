use std::io;
use std::fs::File;
use std::io::Read;
mod new_module; //src/new_module.rs 파일에 정의해 놓은 모듈을 import
use dummy_package_for_check_crate::test_module; //dummy_package_for_check_crate 패키지에 정의해 놓은 모듈을 import. 다른 패키지에 있는 모듈이기 때문에 cargo.toml에 의존성 주입하고 여기서 use로 호출해야함.
use dummy_package_for_check_crate::test_module2;
use dummy_package_for_check_crate::test_module3;
use dummy_package_for_check_crate::task::task_mod::TaskList;
use dummy_package_for_check_crate::task::task_mod::Task;

//result 타입 반환 메서드 -> Ok, Err 반환
fn divide(a:i32, b:i32) -> Result<i32,String>{
    if b == 0 {
        // Err("0으로 나눌 수 없습니다.".to_string()) -> 암시적 반환은 함수의 마지막에만 사용 가능. 현재 상태로는 else문이 따로 없기 때문에 이 if문이 실행되고 나서도 추가적인 실행이 있을 수 있기 때문에, 이 상황에서 암시적 반환을 하면 에러가 발생함.
        return Err("0으로 나눌 수 없습니다.".to_string());
    } 
    Ok(a/b)
}

//option 타입 반환 메서드 -> Some, None 반환
fn get_score(name: &str) -> Option<i32>{
    match name{
        "hhh" => Some(95),
        _ => None,
    }
}


//? 연산자 확인 메서드
fn read_file() -> Result<String, io::Error>{
    let mut file  = File::open("testFile.txt")?;//에러가 발생하면 바로 반환. 즉 호출한 메서드에 Err를 반환
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;//에러가 발생하면 바로 반환
    Ok(contents)
}




fn main() {
    //result -> 성공시 Ok, 실패시 Err return
    let mut input = String::new();
    println!("첫번째 숫자를 입력하세요");
    io::stdin().read_line(&mut input).expect("잘못된 입력");
    let input1:i32 = match input.trim().parse(){
        Ok(n) => {
            println!("정상 처리, 입력값: {}", n);
            n
        }
        Err(e) => {
            println!("오류 발생, 입력값: {}", e);
            return;
        }
    };
    input.clear();
    println!("두번째 숫자를 입력하세요");
    io::stdin().read_line(&mut input).expect("잘못된 입력");
    let input2:i32 = match input.trim().parse(){
        Ok(n) => {
            println!("정상 처리, 입력값: {}", n);
            n
        }
        Err(e) => {
            println!("오류 발생, 입력값: {}", e);
            return;
        }
    };

    match divide(input1,input2){
        Ok(result) => {
            println!("결과: {}", result);
        }
        Err(e) => {
            println!("오류 발생: {}", e);
        }
    }

    let score = get_score("hhh");
    match score{
        Some(s) => println!("점수: {}", s),
        None => println!("점수를 찾을 수 없습니다."),
    }


    //unwarp() : result나 option 타입의 값을 강제로 추출
    let number = "12".parse::<i32>().unwrap(); //콜백 전에 실행된 함수의 결과가 있는 경우 그 값을 반환, 없는 경우 패닉 발생
    println!("숫자: {}", number);


    let numberParse = "12".parse::<i32>().expect("숫자 변환 실패");//콜백 전에 실행된 함수의 결과가 있는 경우 그 값을 반환, 없는 경우 패닉이 발생하지만 expect 파라미터로 적은 문자열이 출력
    println!("숫자: {}", numberParse);



    //? 연산자: 에러를 상위 함수로 전파
    match read_file(){
        Ok(contents) => println!("파일 내용: {}", contents),
        Err(e) => println!("파일 읽기 오류: {}", e),
    }
    

    //모듈 사용
    println!("10 + 5 = {}", new_module::math::add(10,5));
    println!("10 - 5 = {}", new_module::math::sub(10,5));


    //다른 패키지에 있는 모듈 사용
    test_module::say_hello();
    test_module2::say_hello();
    test_module3::say_hello();

    //crate : rustc를 통해 컴파일되는 컴파일 단위
    //crate키워드는 현재 crate의 루트경로를 가리킴. 단순히 절대 경로 키워드
    


    //실습
    //간단한 할 일 관리 프로그램
    let mut task_list = TaskList::new();
    let mut task1 = Task::new("Study Rust".to_string(),"Learn Rust Programming".to_string());
    task_list.add_task(task1)?; //ai 리팩토링. ?연산자로 에러처리
    task_list.get_all_tasks();
}


