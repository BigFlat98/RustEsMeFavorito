use std::i32; //i32 모듈 임포트
use std::collections::HashMap; //hashmap 모듈 임포트

fn main() {
    //Vector -> 동적으로 크기를 할당할 수 있는 같은 타입을 저장할 수 있는 컬랙션
    let mut v1: Vec<i32> = Vec::new(); //vector객체 생성
    let v2 = vec![1,2,3,4,5]; //매크로를 사용한 초기값이 있는 벡터 생성. 불변형으로 선언했기 때문에 기존 요소의 값 변경 불가능. push pop 사용 불가능.
    //let이나 const로 선언하는경우 vector가 아닌 일반 배열로 선언하는게 나아 보이지만 벡터는 힙 영역에 할당되기 때문에 크기가 큰 벡터는 힙에 안전히 할당할 수 있음. 스택은 오버플로우가 발생할 수 있기 때문에 위험.


    v1.push(5); //벡터에 요소 추가
    v1.push(32);
    v1.push(100);

    //벡터 요소 접근
    let third: &i32 = &v2[2]; //인덱스를 사용해 접근 **배열의 특정 인덱스요소를 다른 변수로 소유권을 넘길 수 없음. 작동을 안함. 배열을 다음 행과 전 행이 순서대로 있어야 하기 때문에.
    println!("the third element is{}",third);

    match v2.get(2){//안전하게 get메서드를 사용해 접근. 인덱스로 접근시 panic발생 가능성이 있음.
        Some(third) => println!("the third element is{}",third),
        None => println!("there is no third element"),
    }

    for i in &v1{ //반복문에서 벡터 사용. 파이썬이랑 비슷. 하지만 불변 참조를 사용하는 이유는 소유권 때문임. 그냥 직접 사용하면 소유권이 for문에 넘어감.
        print!("{} ",i);
    }
    println!();
    for i in &mut v1{ //가변 참조를 사용해 벡터 요소 변경 -> 값을 변경시 원래 백터의 요소들도 변경됨.
        
        *i += 50;//값을 변경하는 경우에서 *를 사용해서 참조
        println!("{}",i); //단순히 값을 출력하는 경우 * 붙일 필요 없음.
    }

    for i in &v1{
        println!("{}",i);
    }

    //vector가 적합한 경우 -> 값의 추가 삭제등 변화가 있을 때
    let mut user_scores: Vec<i32> = Vec::new(); //벡터 선언
    user_scores.push(100); //벡터에 요소 추가
    user_scores.push(90);
    user_scores.push(80);

    let first = user_scores[0]; //벡터 요소 접근
    println!("{}",first);
    println!("{}",first);
    user_scores.remove(0);//특정 인덱스 요소 삭제
    user_scores.pop();//마지막 요소 삭제
    let file_contents = vec![0u8; 1024]; //1024개의 요소를 가진 벡터 생성 0u8은 0을 갖는 부호없는 8비트 정수를 나타냄.

    //2차원 벡터
    let mut grid = vec![vec![0; 9]; 9]; //vec![초기로 들어가는 값; 행 개수]
    for i in 0..9{
        for j in 0..9{
            grid[i][j] = i*j;
        }
    }
    for i in 0..9{
        for j in 0..9{
            print!("{} * {} = {} ",i,j,grid[i][j]);
        }
        println!();
    }




    //String -> utf-8로 인코딩된 가변 크기의 문자열
    let mut s1 = String::new(); //빈 문자열 객체 생성
    let s2 = String::from("Hello");//문자열을 리터럴을 사용해 생성
    let s3 = "Hola~".to_string();//문자열 리터럴을 문자열 객체로 변환
    
    //String vs str -> String은 가변 크기 문자열이고 str은 고정 크기 문자열
    let str_literal = "buenos dias!";
    let mut string_type = str_literal.to_string(); //.to_string()은 위에 적어놓은대로 문자열로 새 문자열 객체를 만드는거라 지금같은 대입연산자를 사용했을 때 소유권이 너머가지 않음.
    println!("{}",str_literal);
    println!("{}",string_type);

    string_type.push_str(" que tal");
    string_type.push('?');

    //String 연결
    let hello = String::from("Hello,");
    let world = String::from("world!");
    let hello_world = hello + &world; //문자열의 + 연산자는 신기하게 작동함. string + &str 형태로만 작동함. 다른 형태는 컴파일 에러 발생.
                                              //여기서 hello 변수는 그냥 대입 연산자를 사용하는거기 때문에 소유권이 넘어감.

    let hello_world2 = hello_world.clone() + "que tal?"; //clone()을 사용해 소유권을 넘기지 않음.

    let hh = format!("{} {}",hello_world,hello_world2); //format! 매크로를 사용해 문자열 연결. 둘 다 소유권 유지.

    
    //HashMap -> 키-값 쌍을 저장하는 컬렉션. 파이썬의 딕셔너리와 비슷.
    //모든 키는 같은 타입이어야 함. 모든 값도 같은 타입이어야 함.
    let mut scores= HashMap::new(); //HashMap 객체 생성

    //값 삽입
    scores.insert(String::from("Blue"),10); //키 - 값 삽입.
    scores.insert(String::from("Red"),50);
    scores.insert(String::from("Blue"),100); //같은 키를 사용해 값 삽입시 기존 값 덮어씌움.
    scores.insert(String::from("Green"),100);

    //값 조회
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score{
        Some(s) => println!("{}",s),
        None => println!("no score"),
    }
    //if let 사용해서 Option 타입 간단히 처리
    if let Some(score) = scores.get(&team_name){
        println!("{}",score);
    }else{
        println!("no score");
    }


    //키-값 쌍 반복
    for (key,value) in &scores{
        println!("{}: {}",key,value);
    }


    //키-값 쌍 삭제
    scores.remove(&team_name);
    

    //collect 메서드를 사용해 생성
    let teams = vec![String::from("Blue"),String::from("Red")];
    let initial_scores = vec![10,50];
    let scores: HashMap<_,_> = teams.iter().zip(initial_scores.iter()).collect(); //zip 메서드를 사용해 두 벡터를 합침. 두 벡터의 요소 개수가 같아야 함.


    //타입 명시적 선언
    let mut grades: HashMap<String,i32> = HashMap::new();
    grades.insert(String::from("Maven"),98);
    grades.insert(String::from("John"),87);
    grades.insert(String::from("Jane"),95);
    grades.insert(String::from("Diana"),88);


    //entry 메서드를 사용해 키-값 쌍 삽입
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.entry(String::from("Yellow")).or_insert(50); //entry 메서드를 사용해 키-값 쌍 삽입. 키가 존재하지 않으면 값 삽입.
    scores.entry(String::from("Blue")).or_insert(50); //키가 존재하면 값 삽입 안함.


    //간단한 방법
    let map = HashMap::from([("Blue",10),("Red",50)]); //키-값 쌍을 튜플 벡터로 전달해 생성.
}
