use std::fs::File; //파일 관련 기능 제공
use std::io; //입출력 관련 기능 제공
use std::io::Read; //파일 읽기 관련 기능 제공


fn main() {
    //Result 타입, option 처럼 자동으로 불러옴
    //enum Result<T,E>{
    //    Ok(T),
    //    Err(E),
    //}

    //기본적인 result 처리
    let result: Result<i32,String> = Ok(10);
    let result2: Result<i32,String> = Err("error".to_string());

    //unwarp - 성공 값 추출 또는 패닉
    let value1 =result.clone().unwrap();

    //expect() - 커스텀 메세지와 함께 unwarp
    let value2 = result.clone().expect("custom error message"); 
    //clone을 안쓰니까 여기서 소유권 넘어가서 뒤에서 result를 못씀

    // unwrap_or() - 성공 값 추출 또는 기본값 반환
    let value3 = result2.clone().unwrap_or(0);

    //unwrap_or_else() - 성공 값 추출 또는 함수 호출 결과 반환
    let value4 = result2.clone().unwrap_or_else(|err|{
        println!("custom error message");
        0
    });

    //match로 직접 처리
    let value5 = match result {
        Ok(val) => val,
        Err(err) =>{
            println!("error: {}",err);
            0
        }
    };

    println!("value1: {}",value1);
    println!("value2: {}",value2);
    println!("value3: {}",value3);
    println!("value4: {}",value4);
    println!("value5: {}",value5);



    //실사용 예제
    //파일 처리
    let file_result = File::open("test.txt");//파일 열기 성공시 ok, 실패시 err 반환
    match file_result{
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();//파일을 읽어 contents에 저장
            println!("file contents: {}", contents);
        }//파일 읽기 성공시 
        Err(err) => {
            println!("error: {}",err);
        }
    }

    //숫자 반환
    let number_str = "42";
    let parsed: Result<i32,_> = number_str.parse(); //parse 메서드는 문자열을 숫자로 변환하는 메서드. 성공시 ok<i32>, 실패시 err<io::Error> 반환
    if let Ok(n) = parsed { //if let 구문으로 성공시 결과 추출
        println!("parsed number: {}",n);
    }

    //? 연산자 사용 -> ?연산자는 result 타입에서 사용하는 연산자. 성공시 값 반환, 실패시 즉시 함수 종료
    fn read_file() -> Result<String,io::Error>{
        let mut file = File::open("test.txt")?; //성공시 file 변수에 파일 핸들러 할당, 실패시 즉시 함수 종료
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; //성공시 contents에 파일 내용 저장, 실패시 즉시 함수 종료
        Ok(contents)
    }


    //panic! 사용
    let v = vec![1,2,3];
    //v[10]; 패닉 발생. 패닉은 치명적인 오류가 발생했을 때 프로그램음 안정적으로 종료하는 기능.
    //코딩하는 과정에서 특정 상황에서 문제가 발생했을 때 해당 상황에서 더이상 실행되지 않아야 하는 경우 발생시킴 panic!("메세지"); 로 실행할 수 있음.


    //에러전파
    //함수 내에서 발행한 에러를 직접 처리하지 않고, 호출한 쪽으로 전달하는 기법. 주로 result타입과 ?연산자를 사용해 에러를 전파함
    //위쪽에서 함.

}