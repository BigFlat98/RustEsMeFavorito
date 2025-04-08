//실습
use std::io;
struct Student{
    name:String,
    age:u32,
    scores:Vec<u32>,
}
impl Student{
    fn new(name:String,age:u32,scores:Vec<u32>)->Self{
        Student{name,age,scores}
    }

    fn print_info(&self){
        println!("학생 정보:");
        println!("이름: {}",self.name);
        println!("나이: {}",self.age);
    }
    fn print_grade(&self){
        println!("성적 정보:");
        for i in 0..self.scores.len(){
            match i {
                0 => {print!("국어: {}",self.scores[i]);
                    Grade::get_grade(&self.scores[i]).check_grade(); //get_grade 메서드를 인스턴스 생성 없이 바로 썼는데, 마치 자바의 정적 메서드 같음. 러스트에서는 연과 함수라고 부른다고 함.
                },
                1 => {print!("수학: {}",self.scores[i]);
                    Grade::get_grade(&self.scores[i]).check_grade();
                },
                2 => {print!("영어: {}",self.scores[i]);
                    Grade::get_grade(&self.scores[i]).check_grade();
                },
                _ => println!("기타"),
            }
        }
        println!("평균: {}",self.scores.iter().sum::<u32>()/self.scores.len() as u32); //이 평균 메서드 기깔난 것 같음.iter()과 sum메서드와 len메서드를 활용해서 만들어서
        
    }
}
enum Grade{
    A,
    B,
    C,
    D,
    F,
}
impl Grade{
    fn get_grade(score:&u32)->Self{
        match score{
            90..=100 => Grade::A,
            80..=89 => Grade::B,
            70..=79 => Grade::C,
            60..=69 => Grade::D,
            _ => Grade::F,
        }
    }
    fn check_grade(&self){
        match self{
            Grade::A => println!("(A) 최우수"),
            Grade::B => println!("(B) 우수"),
            Grade::C => println!("(C) 보통"),
            Grade::D => println!("(D) 부족"),
            Grade::F => println!("(F) 불합격"),
        }
    }
    
}



// Option은 표준 라이브러리에 이미 정의되어 있음
// enum Option<T> { 제네릭을 활용해 T에 어떤 타입의 데이터가 들어와도 Some을 기준으로 Match를 사용할 수 있음.
//     Some(T),  // 값이 있는 경우
//     None,     // 값이 없는 경우. 기존에 null이 갖던 문제를 해결하기 위해 도입. null대신 None을 반환하도록 코딩이 필요함.
// }


//구조체 정의
struct Person{
    name:String,
    age:u32,
    height:f64,
}
//구조체 메서드 추가(이 기능은 객체지향언어가 아닌 Rust를 객체지향언어처럼 사용할 수 있게 해줌)
impl Person{
    fn new(name:String,age:u32,height:f64)->Self{//생성자 메서드
        Person{name,age,height}
    }
    fn print_info(&self){//불변 참조자로 self참조
        println!("이름: {}",self.name);
        println!("나이: {}",self.age);
        println!("키: {}",self.height);
    }

    fn self_match(&self){
        match self.age{
            0..=18 => println!("미성년자"),
            19..=100 => println!("성인"),
            _ => println!("노인"),
        }
    }

    fn find_hhh(&self){
        match self{
            Person{name:n,age:a,height:h} if n == "hhh" => println!("이름: {}, 나이: {}, 키: {}",n,a,h), //패턴 가드
            _ => println!("hhh가 아닙니다."),
        }
    }
}




//열거형 : 가능한 값들의 집합을 정의
enum Direction{
    North,
    South,
    East,
    West,
}
//데이터를 포함한 열거형
enum Message{
    Quit,
    Mover{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
//구조체처럼 열거형에도 메서드를 추가할 수 있음.
impl Message{
    fn result_by_self(&self){
        match self{
            Message::Quit => println!("quit"),
            Message::Mover{x,y} => println!("move to {},{}",x,y),
            Message::Write(s) => println!("write: {}",s),
            Message::ChangeColor(r,g,b) => println!("change color to {},{},{}",r,g,b),
        }
    }
}
fn result_message(msg:&Message){ //메서드 만들때도 인자에 참조를 사용하는 습관을 들여야 할 것 같음. 소유권이 넘어가지 않도록 주의해야함.
    match msg{
        Message::Quit => println!("quit"),
        Message::Mover{x,y} => println!("move to {},{}",x,y),
        Message::Write(s) => println!("write: {}",s),
        Message::ChangeColor(r,g,b) => println!("change color to {},{},{}",r,g,b),
    }
}




fn main() {
    let person01 = Person{ //구조체 인스턴스 생성
        name:String::from("hhh"),
        age:30,
        height:176.5,
    };

    //인스턴스 필드에 접근
    println!("이름: {}",person01.name);

    let mut person02 = Person{
        name:String::from("ttt"),
        age:28,
        height:177.5,
    };
    person02.age = 29;
    println!("person02의 나이: {}",person02.age);
    let person03 = Person::new(String::from("ccc"),35,180.0);
    person03.print_info();







    //열거형 패턴 사용
    let dir = Direction::North;//열거형 변수 생성
    match dir{//match 사용
        Direction::North => println!("북쪽"),
        _ => println!("남쪽, 동쪽, 서쪽 셋중 하나"),
    }

    let msg = Message::Write(String::from("Hola~"));
    result_message(&msg); //항상 함수를 사용하는 습관을 들이자! 메서드에서 변수 사용시 참조를 잘 써줘야함. 파라미터 값을 참조로 안쓰면 메서드에 소유권이 넘어가 파라미터로 사용한 변수를 사용할 수 없게 됨.
    msg.result_by_self(); //열거형도 구조체처럼 메서드를 추가해서 사용할 수 있음.



    //Option 타입 사용
    let some_number = Some(32);
    let none_number: Option<i32> = None; //기본적으로 none_number 선언할 때 Option 타입을 명시하는게 좋음
    match some_number{
        Some(n) => println!("숫자: {}",n),
        None => println!("숫자가 없습니다."),
    }
    match none_number{
        Some(n) => println!("숫자: {}",n),
        None => println!("숫자가 없습니다."),
    }
    




    //패턴 매칭 연습
    let x = 1;
    match x{
        //단일 값 매칭
        12 => println!("12"),

        //여러 값 매칭
        30 | 31 => println!("30 또는 31"),

        //범위 매칭
        1..=10 => println!("1부터 10 사이의 숫자"),

        //특정 상정 외 숫자 매칭
        n => println!("{}는 기타 숫자",n),

        //_ 패턴 : 매칭되지 않는 모든 값에 대한 처리
        _ => println!("기타"),
        
    }

    //구조체에서 패턴 매칭
    person01.find_hhh();
    person02.find_hhh();
    person03.self_match();

    //if let을 사용
    if let Some(n) = some_number{
        println!("if let을 사용한 숫자: {}",n);
    }
    if let None = none_number{
        println!("if let을 사용한 숫자가 없습니다.");
    }

    //while let을 사용
    let mut stack = Vec::new();
    stack.push("1");
    stack.push("2");
    stack.push("3");
    while let Some(top) = stack.pop(){//pop은 마지막 요소를 꺼내서 반환. 마지막 인데스는 삭제됨.
        println!("stack에서 꺼낸 값: {}",top);
    }
    

    //@바인딩(at 바인딩) -> 패턴 매칭을 진행하면서 특정 패턴에 매칭되는 경우 그 값을 변수에 저장
    let numbers = (1,2,3,4,5);
    match numbers{
        (first, .., last @ 32) => {
            println!("first: {}, last: {}",first,last);
        },
        _ => println!("기타"),
    }

    

    //실습 
    let mut input_data = String::new();
    println!("학생의 이름 입력");
    io::stdin().read_line(&mut input_data).expect("입력 오류");
    let name:String = input_data.trim().to_string(); // to_string은 Result타입을 반환하지 않기 때문에 match 사용을 해도 타입에러가 발생함.
    input_data.clear();//다음 입력을 위한 초기화가 필요함.
    println!("학생의 나이 입력");
    io::stdin().read_line(&mut input_data).expect("입력 오류");
    let age:u32 = match input_data.trim().parse(){
        Ok(n) => {
            if n < 0 || n > 120{
                println!("나이는 0~120 사이의 숫자로 입력해주세요.");
                return;
            }
            n
        },
        Err(_) => {
            println!("나이는 숫자로 입력해주세요.");
            return;
        }
    };
    input_data.clear();
    let mut scores:Vec<u32> = Vec::new();
    for i in 0..=2{
        println!("{}번째 과목의 점수 입력",i+1);
        io::stdin().read_line(&mut input_data).expect("입력 오류");
        match input_data.trim().parse(){
            Ok(n) =>{
                if n < 0 || n > 100{
                    println!("점수는 0~100 사이의 숫자로 입력해주세요.");
                    return;
                }
                scores.push(n);
            },
            Err(_) => {
                println!("점수는 숫자로 입력해주세요.");
                return;
            }
        }
        input_data.clear();
    }
    let student01 = Student::new(name,age,scores);
    student01.print_info();
    student01.print_grade();
    
    
}
