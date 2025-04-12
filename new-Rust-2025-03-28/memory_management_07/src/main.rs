
//불변 참조를 받는 함수
fn calculate_length(s: &String) ->usize{
    s.len()
}

//가변 참조를 받는 함수
fn append_world(s:&mut String){
    s.push_str(" !!");
}


fn main() {
    //각 값의 하나의 소유권만 갖을 수 있음
    let s1 = String::from("test");
    let s2 = s1; //move : 소유권 이동
    let s3 = s2.clone(); //clone()을 통한 명시적 복사
    //println!("{}",s1); 에러발생!

    //copy trait를 구현한 타입은 복사가 가능함
    let x = 5;
    let y = x;
    println!("x:{},y:{}",x,y);


    //스택 영역에 저장: 고정 크기 데이터
    let z = 30;

    //힙 영역에 저장: 가변 크기 데이터
    let s = String::from("test");

    //vector도 힙에 저장
    let v = vec![1,2,3];


    //불변 참조
    let mut st = String::from("hello!");
    let r1 = &st;
    let r2 = &st;
    //let r3 = &mut st; -> 에러! 불변 참조값이 있는 동안 가변참조 불가능
    println!("r1:{}, r2:{}",r1,r2);//둘 다 참조가능
    let r4 = &mut st;
    r4.push_str(" world~"); //불변 참조 변수 사용 후, 가변 참조 가능

    //가변 참조
    let mut stm = String::from("hola!");
    let r3 = &mut stm;
    r3.push_str(" mucho gusto");
    println!("{}",r3);



    //메서드 참조
    let mut x = String::from("hello!");
    let m_len = calculate_length(&s);

    println!("len : {}",m_len);

    //가변 참조 호출
    append_world(&mut x);
    println!("수정 : {}",x);






    //라이프타임 : 참조가 얼마나 오래 유효한지 컴파일러에게 알려주는 규칙
    let x = 5;            // ----------+-- 'b
    let r = &x;           // --+-- 'a  |
    println!("r: {}", r); //   |       |
    // 'a: r의 라이프타임
    // 'b: x의 라이프타임
    // 참조 r의 라이프타임('a)은 x의 라이프타임('b)보다 짧아야 함




    
}


    // 컴파일 에러 발생하는 코드
// fn longest(x: &str, y: &str) -> &str { //expected named lifetime parameter 에러 발생
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// } x와 y중 어떤 변수가 더 오래 함수에서 살아있는지 구별이 불가능하기 때문임.

//라이프타임의 명확한 명시가 필요함.
fn longest<'a>(x: &'a str, y:&'a str) ->&'a str{ //'a는 관례적인 라이프타임 매개변수 명
    if x.len() > y.len() {
        x
    } else {
        y
    }
}//이건 x와 y는 최고 'a 만큼 살아야 한다는 의미임

//라이프사이클은 실제 메모리 해제하거나 관리하지 않음. 단지 참조의 생명 주기를 암시할 뿐임
//대부분 러스트가 자동으로 추론, 복잡한 상황에 한에서만 명시가 필요

// 라이프타임 명시가 필요한 경우:
// 함수가 참조를 반환할 때
// 구조체가 참조를 포함할 때
// 여러 참조의 라이프타임 관계를 명확히 해야 할 때
// 라이프타임 명시 규칙:
// 꺾쇠괄호 <>안에 라이프타임 매개변수 선언
// 라이프타임은 작은따옴표 '로 시작
// 보통 소문자 알파벳 사용 ('a, 'b, 'c)
// 참조 타입 앞에 라이프타임 표시 (&'a str)

// 아래 세 가지 경우는 라이프타임을 생략할 수 있습니다
// 1. 각 참조 매개변수는 고유한 라이프타임을 가짐
// 원래 형태
fn print_both1<'a, 'b>(x: &'a str, y: &'b str) {
    println!("x: {}, y: {}", x, y);
}
// 생략 가능
fn print_both2(x: &str, y: &str) {
    println!("x: {}, y: {}", x, y);
}
//각 매개변수가 서로 관계없이 독립적으로 사용되므로 라이프타임을 생략할 수 있습니다.

// 2. 입력 라이프타임이 하나면 출력도 같은 라이프타임
// 원래 형태
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// 생략 가능
// fn first_word(s: &str) -> &str {
//     // ... 동일한 구현 ...
// }
//입력 참조가 하나(&str)이므로 출력 참조도 같은 라이프타임을 가진다고 컴파일러가 추론합니다.

// 3. 메서드의 &self면 모든 출력이 self의 라이프타임을 가짐
struct StringHolder<'a> {
    content: &'a str,
}

// 원래 형태
impl<'a> StringHolder<'a> {
    fn get_content<'b>(&'b self) -> &'a str {
        self.content
    }
}

// 생략 가능
impl StringHolder<'_> {
    fn get_content(&self) -> &str {
        self.content
    }
}
//메서드가 &self를 받으면 반환되는 참조는 self의 라이프타임을 따른다고 컴파일러가 추론합니다.


