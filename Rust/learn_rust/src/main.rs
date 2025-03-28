
//구조체
struct FirstStruct{
    x: i32,
    y: f64,
    z: String,
    w: bool,
    a: char,
    v: [i32; 5],
    t: (i64, i32, String),
}

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{

    //생성자 메서드 (관례적으로 new라고 부름)
    fn new(width: u32, height: u32)-> Rectangle{
        Rectangle {width, height}
    }

    //메서드
    fn area(&self)-> u32{ //self는 인스턴스 자신을 가리키는 포인터 this와 유사.
        self.width * self.height
    }

    fn square(size: u32)->Rectangle{
        Rectangle {width: size, height: size}
    }

    //소유권을 가져가는 메서드
    //메서드 실행 후 인스턴스가 제거됨.
    fn delete(self){//함수가 인스턴스의 소유권을 가져갔다 종료되면 기존 인스턴스를 갖는 변수는 소유권을 잃고 메모리 해제.
        println!("delete is called");
    }
}


//열거형 (가능한 값들의 집합 정의)
enum Message {
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(u8,u8,u8),
}

impl Message{
    fn call(&self){
        match self{ //match는 열거형의 값을 확인하고 해당 값에 대한 처리를 수행하는 구문
                    //switch와 유사. Message::Quit 값을 가진 인스턴스를 통해 call 메서드 호출 시 Quit을 출력.
            Message::Quit => println!("Quit"),
            Message::Move{x,y} => println!("Move to ({},{})",x,y),
            Message::Write(text) => println!("Write: {}",text),
            Message::ChangeColor(r,g,b) => println!("Change color to ({},{},{})",r,g,b),
        }
    }
}

//제네릭
fn print_value<T>(value:T) -> T{//제네릭<>은 안에 들어오는 변수의 타입을 저장함.
    //이게 기깔 나는게 뭐냐.. 만약 제네릭이 없으면 같은 방식의 동작을 하는 함수지만 타입이 다르면 각각 구현해 줘야함.
    //하지만 제네릭을 사용하면 변수의 타입을 정하 않고 들어오는 변수의 타입 그대로 사용가능.
    return value;
}


fn main() {
    let x =10;
    let y = 20;

    println!("{}", add(x,y));
    println!("{}", sub(x,y));
    println!("{}", mul(x,y));
    println!("{}", div(x,y));
    println!("{}", moddd(x,y));




    //조건문
    let number = 20;
    if number > 5 {
        println!("number is bigger then 5");
    }
    else if number == 5 {
        println!("number is same with 5");
    }
    else{
        println!("number is less than 5");
    }




    //반복문
    let numbers: [i32; 5] = [1,2,3,4,5];
    for number in numbers.iter() {
        println!("The number is : {}",number);
    }

    let mut count: i32 = 0;
    loop {
        count += 1;
        println!("count is {}",count);
        if count == 10 {
            break;
        }
    }
    
    while count > 2 {
        println!("count is {}",count);
        count -= 1;
    }
    println!("count is {}",count);


    for number in 1..=10 {
        println!("number is {}",number);
    }




    //소유권
    let s1 = String::from("hello");
    let s2 = s1; //s1의 소유권이 s2로 이동. s1은 메모리 해제
    //println!("s1 is {}",s1); 오류 발생!
    println!("s2 is {}",s2);

    //복제(clone) 
    let s3 = String::from("hola");
    let s4 = s3.clone(); //s3의 복제본을 생성
    println!("s3 is {}",s3);
    println!("s4 is {}",s4);

    //참조(대여)
    let mut s = String::from("buenos ");
    //불변 참조
    let r1 = &s;//s를 한번에 여러번 불변 참조에 대해 대여해줄 수 있음
    let r2 = &s;
    // let r3 = &mut s; 오류 발생! -> 불변 참조가 끝나기 전에 가변 참조를 할 수 없음
    println!("r1 is {}, r2 is {}",r1,r2); //불변참조한 변수 사용 완료

    //가변 참조
    let r3 = &mut s; //대여해준 변수들 사용이 끝났기 때문에 가변 참조 가능.
    r3.push_str("dias");//r3수정시 s도 수정됨
    println!("r3 is {}",r3); 
    println!("s is {}",s);
    //참조 규칙 -> 불변 참조는 한번에 여러개 가능. 가변 참조는 불변 참조한 변수를 모두 사용한 후에 가능.

    //구조체 
    let NewStruct = FirstStruct {
        x: 10,
        y: 20.0,
        z: String::from("hello"),
        w: true,
        a: 'a',
        v: [1,2,3,4,5],
        t: (10,20,String::from("hello")),
    };//인스턴스 생성시 모든 필드롤 초기화 해줘야 함
    println!("NewStruct is {}",NewStruct.z);

    let rect1 = Rectangle::new(10,20);//생성자 메서드 호출
    println!("rect1 area is {}",rect1.area());

    let rect3 = Rectangle::new(30,16);
    println!("rect3 area is {}",rect3.area());

    let rect2 = Rectangle::square(10);//정사각형 생성
    println!("rect2 area is {}",rect2.area());




    //열거형 구조체
    let mut m = Message::Move{x:10,y:30};
    m = Message::Quit;

    m.call();

}
fn add(x: i32, y:i32) -> i32{
    x + y
}

fn sub(x: i32, y:i32) -> i32{
    x - y
}

fn mul(x: i32, y:i32) -> i32{
    x * y
}

fn div(x: i32, y:i32) -> i32{
    x / y
}

fn moddd(x: i32, y:i32) -> i32{
    x % y
}

