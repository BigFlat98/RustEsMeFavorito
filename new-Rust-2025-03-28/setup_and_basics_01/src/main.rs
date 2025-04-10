use std::io; //표준 입출력 라이브러리 불러오기

fn main() {
    // 변수 선언(기본적으로 불변)
    let x = 5;
    
    //가변 변수
    let mut y = 100;
    y = 15;

    const PI: f64 = 3.14;

    //기본 데이터 타입
    let integer: i32 = 4;
    let float: f64 = 8.0;
    let boolean: bool = true;
    let character: char = 'R';

    //연산자
    let sum = x + y;
    let diff = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;

    println!("합: {}", sum);
    println!("차: {}", diff);
    println!("곱: {}", product);
    println!("몫: {}", quotient);
    println!("나머지: {}", remainder);
    
    
    
    //문자열
    let string = String::from("hello!");
    let strt_slice: &str = "Rust~~";

    //튜플
    let tuple: (i32, f64, char) = (3,3.14,'R');
    let (a,b,c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);

    //배열
    let array: [i32; 5] =[1,2,3,4,5];


    //문자열 입력 받기
    println!("이름을 입력하세요:");
    let mut input = String::new(); //가변 문자열 변수 생성
    io::stdin().read_line(&mut input).expect("입력 오류"); //가변 참조해서 입력값 저장, 입력 오류 처리

    let input = input.trim(); //공백 제거
    println!("입력한 이름: {}", input);


    println!("나이를 입력하세요:");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("입력 오류");
    let converted_age: u32 = match age.trim().parse(){//parse 메서드는 문자열을 숫자로 변환
        Ok(num) => num,
        Err(_) =>{
            println!("나이를 숫자로 입력해주세요.");
            0 //0으로 반환
        }
    };

    println!("나이: {}", converted_age);


    //실습 1) 원뿔의 부피 계산기
    println!("원뿔의 반지름을 입력하세요:");
    let mut radius = String::new();
    io::stdin().read_line(&mut radius).expect("잘못된 입력");
    let radius: f64 = match radius.trim().parse(){
        Ok(num) => num,
        Err(_) =>{
            println!("숫자를 입력해주세요.");
            0.0
        }
    };

    println!("원뿔의 높이를 입력하세요:");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("잘못된 입력");
    let height: f64 = match height.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("숫자를 입력해주세요.");
            0.0
        }
    };

    let volume = cal_circle(radius, height);
    println!("원뿔의 부피: {}", volume);


    
}

fn add(a: i32, b: i32) -> i32{
    a+b //세미콜론 없으면 반환값
}

fn greet(name: &str){
    println!("Hello, {}!", name);
}

fn cal_circle(radius: f64, height: f64) -> f64{
    let volume = (3.14 * radius * radius * height) / 3.0;
    volume
}