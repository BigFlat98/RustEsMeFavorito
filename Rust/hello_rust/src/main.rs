fn main() {
    println!("Hello, world!");

    //변수 선언(기본적으로 초기화 불가)
    let x = 10;
    
    //가변 변수 선언
    let mut y = 20;
    println!("y: {}",y);
    y = 30;
    println!("y: {}",y);

    //기본 데이터 타입
    let a: i32 = 16; // 32비트 정수
    let b: i64 = 1234567890; // 64비트 정수
    let c: f32 = 3.14; // 32비트 실수
    let d: f64 = 3.141592653589793; // 64비트 실수
    let e: bool = true; // 불리언
    let f: char = 'R'; // 문자
    let g: &str = "Hello, Rust"; // 문자열 
    let h: String = String::from("Hello, Rust");// 문자열 &str형 문자열은 가변 불가, 참조형 문자열임. 하지만 String형은 가변 가능.
    let i: u8 = 255;// 8비트 부호없는 정수
    let j: u16 = 65535; // 16비트 부호없는 정수
    let k: u32 = 4294967295; // 32비트 부호없는 정수
    let l: u64 = 18446744073709551615; // 64비트 부호없는 정수

    println!("a: {}", g);
    println!("{}", transTemp(27));

    let mut newString: String = String::from("New String");
    println!("{}", newString);
    newString.push_str("change String");
    println!("{}",newString);
    


}

fn add(x: i32, y: i32) -> i32{
    x + y // ;세미콜론 찍으면 오류남. 컴파일 에러 -> 그냥 () 반환
}

fn add2(x: i32, y: i32) -> i32{
    return x + y; // 이렇게 명시적으로 return 써주면 됨.
}

fn transTemp(x: i32) -> i32{
    return (x - 32) * 5 / 9;
}


