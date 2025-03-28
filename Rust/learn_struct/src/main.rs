fn main() {
    //기본적인 열거형 정의
    enum Message{
        Quit,
        Move{x:i32,y:i32},
        Write(String),
        ChangeColor(i32,i32,i32),
    }

    //열거형 사용
    let msg1 = Message::Quit; //변수에 특정 열거형 값 할당.
    let msg2 = Message::Move{x:10,y:20};
    let msg3 = Message::Write(String::from("Hello,Rust!"));
    let msg4 = Message::ChangeColor(255,255,255);

    //열거형 값 패턴 매칭
    match msg1{ //if문과 유사해 보이나 좀 더 강력함.
        //완전성 -> 열거형에 적혀있는 모든 경우별로 처리 작업을 해야함.(만약 Quit에 대한 처리작업이 없으면 컴파일러가 에러를 발생시킴)
        //이로 인해 특정 경우를 놓치는 상황을 예방할 수 있음.
        Message::Quit => println!("Quit"),
        Message::Move{x,y} => println!("Move to ({},{})",x,y),
        Message::Write(text) => println!("Write: {}",text),
        Message::ChangeColor(r,g,b) => println!("ChangeColor: ({},{},{})",r,g,b),
        _ => println!("Unknown message"), //_(와일드카드)는 모든 경우를 처리하는 패턴.(if의 else와 같은 역할)
    }

    match msg2{
        Message::Move{x,y} => println!("Move to ({},{})",x,y),
        _ => println!("Unknown message"),//와일드 카드를 사용해 나머지 열거형 값을 포함해서 처리할 수 있음.
    }

    if let Message::Move{x,y} = msg2{ //if let문은 특정 패턴에 대한 처리작업을 간결하게 할 수 있음.
        //여기서 =은 대입 연산자가 아닌 패턴 매칭 연산자로 작동함.
        println!("Move to ({},{})",x,y);
        //if let문은 하나의 패턴에 대해 간단한 처리가 가능하지만 복잡한 패턴을 처리하기엔 부족함.
    }






    //옵션 열거형
    //기본 구조
    //이미 표준 라이브러리에 정의돼있음. 추가적으로 정의하면 에러 발생.
    //너무 자주 사용되기 때문에 import하지 않아도 자동으로 불러옴.
    // enum Option<T>{
    //     Some(T),//값이 있는 경우
    //     None,//값이 없는 경우
    // } -> 이미 정의 돼있으니까 그냥 사용하면 됨.

    

    //옵션 열거형 사용
    let some_number = Some(5);
    let no_number: Option<i32> = None; //option열거형을 사용하는 이유 중 하나. Rust는 Null값을 허용하지 않음.
                                       //때문에 값이 없는 경우를 표현하기 위한 문법이 필요. 

    match some_number{
        Some(value) => println!("Some number: {}",value),
        None => println!("No number"),
    }

    match no_number{
        Some(value) => println!("Some number: {}",value),
        None => println!("No number"),
    }

    //옵션 열거형 패턴 매칭 간결하게 사용
    if let Some(value) = some_number{
        println!("Some number: {}",value);
    }

    

    

}
