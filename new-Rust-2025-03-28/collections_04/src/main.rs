use std::collections::HashMap;
use std::io;



fn main() {
    //Vector -> 동적 배열, 같은 타입의 값들을 저장할 수 있음
    let mut v1: Vec<i32> = Vec::new();//빈 벡터 생성
    let v2 = vec![1,2,3]; //메크로로 초기값과 함께 생성

    v1.push(1);
    v1.push(2);
    v1.push(3);

    let third = &v1[2];//인덱스로 접근(범위를 벗어나면 패닉)
    println!("The third element is {}", third);

    //get 메서드를 통한 안전한 접근
    match v1.get(3){//get 메서드는 Result를 반환하고 만약 해당 인덱스에 값이 없는경우 None을 반환하기 때문에 out of bounds 오류를 예방할 수 있음.
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 벡터 순회
    for i in &v1{
        print!("{} ", i);
    }
    println!();

    // 가변 참조로 순회하며 값 수정
    for i in &mut v1{
        *i += 50; //역참조: c++의 포인터처럼 실제 값을 참조 i 는 실제로는 지금 벡터 인덱스중 하나의 주소를 가지고 있기 때문에 인덱스 안에 있는 값을 초기화 하기 위해서는 역참조를 해서 값에 직접 접근해야함.
        print!("{} ", i);
    }

    println!("uno, dos index: {:?}", &v1[0..=1]);

    v1.pop();//마지막 요소 제거
    println!("v1: {:?}", v1);
    
    




    //문자열 -> utf-8로 인코딩된 바이트의 컬럭션
    let mut s1 = String::new();
    let s2 = "zapatos".to_string();
    let s3 = String::from("aburrido");

    //문자열 추가
    s1.push_str("hola"); //문자열 추가
    s1.push(' '); //문자 추가
    s1.push_str("mucho gusto!");

    //문자열 결합
    let s4 = s2 + &s3; //s2는 소유권이 넘어가고 s3는 참조로 전달되어 소유권이 변경되지 않음.
    println!("{}", s4);
    let s5 = format!("{} {} {}", s1, s3, s4); //format! 매크로를 사용하여 여러 문자열을 결합하고 새로운 문자열을 생성. 사용하는 변수들의 소유권은 유지.
    //format! 메크로는 지금처럼 문자열과 벡터를 함께 사용하는 등으로 활용할 수 있음.

    // 문자열 슬라이스
    let hello = "안녕하세요";
    //**주의** let s = &hello[2..4]; -> 문자열 슬라이스는 바이트 단위로 작동하기 때문에 한글 문자열을 슬라이스 할 경우 예상치 못한 결과가 나올 수 있음.
    // &hello[0..3]   // 0,1,2 바이트 선택 ("안")
    // &hello[3..6]   // 3,4,5 바이트 선택 ("녕")
    // &hello[6..9]   // 6,7,8 바이트 선택 ("하")
    // &hello[..3]    // 처음부터 3번째 바이트까지 ("안")
    // &hello[3..]    // 3번째 바이트부터 끝까지 ("녕하세요")
    
    for c in hello.chars(){//문자열을 글자 단위로 분리
        println!("{}", c);
    }

    for b in hello.bytes(){//문자열을 바이트 단위로 분리
        println!("{}", b);
    }




    //해시맵 -> 키-값 쌍으로 데이터를 저장하는 자료구조
    let mut scores = HashMap::new();
    
    //값 추가
    scores.insert(String::from("azul"),10);
    scores.insert(String::from("marron"),20);

    //값 접근
    let team_name = String::from("azul");
    let score = scores.get(&team_name);

    // 값 순회
    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }

    //값 업데이트
    scores.insert(String::from("azul"),30); //덮어쓰기
    scores.entry(String::from("marron")).or_insert(80); //entry 메서드로 해쉬맵에서 파라미터로 넣은 키값을 갖는 요소를 가져옴. or_insert는 키가 없을 때 : 새로운 요소 생성, 키가 있으면 기존 키의 값 반환
    println!("{:?}",scores);

    //기존 값 기반으로 업데이트
    let text = "Disculpe, SeÑor. como esta usted?";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){//공백 기준으로 분리
        map.entry(word).or_insert(());//존재하지 않는 키에 대해서만 값 추가
    }
    println!("{:?}", map);





    //실습
    let mut students: HashMap<String, Vec<u8>> = HashMap::new();
    let mut input = String::new();
    let mut count:u32 = 1;
    let clase_name:Vec<String> = vec![String::from("국어"), String::from("수학"), String::from("영어")];
    let mut student_input_score:Vec<u8> = Vec::new();
    loop{
        input.clear();
        println!("학생 관리 프로그램");
        println!("1. 학생 추가");
        println!("2. 학생 제거");
        println!("3. 학생 검색");
        println!("4. 학생 전체 조회");
        println!("5. 종료");
        println!("원하는 작업을 선택하세요(번호를 입력하세요): ");
        io::stdin().read_line(&mut input).expect("입력 오류");
        let choice:u8 = match input.trim().parse(){
            Ok(num) => {
                if num >= 1 && num <= 5{
                    num
                }else{
                    println!("1~5 사이의 숫자를 입력해주세요.");
                    continue;
                }
            }
            Err(_) => {
                println!("유효한 숫자를 입력해주세요.");
                continue;
            }
        };
        println!("선택된 작업: {}", choice);
        input.clear();
        match choice{
            1 => {
                println!("학생 추가");
                println!("이름을 입력해주세요: ");
                io::stdin().read_line(&mut input).expect("입력 오류");
                let name = input.trim().to_string();
                println!("성적을 입력해주세요: ");
                for i in 0..3{
                    input.clear();
                    println!("{}과목 성적: ", clase_name[i]);
                    io::stdin().read_line(&mut input).expect("입력 오류");
                    let score:u8 = match input.trim().parse(){
                        Ok(num) => {
                            if num >= 0 && num <= 100{
                                num
                            }else{
                                println!("0~100 사이의 숫자를 입력해주세요.");
                                continue;
                            }
                        },
                        Err(_) => {
                            println!("유효한 숫자를 입력해주세요.");
                            continue;
                        }
                    };
                    student_input_score.push(score);
                    
                }
                students.insert(name.clone(), student_input_score.clone());
                //일단 insert 메서드에서 사용되는 파라미터의 변수들은 반드시 소유권을 뺐김. insert 메서드가 필요로 함. 
                //때문에 &참조형을 쓰면 에러가 발생하고, 소유권을 넘기기 싫다면 클론을 사용해서 복사본을 만들어 소유권을 넘겨야 함.


                student_input_score.clear();
                println!("학생 추가 완료");     
            },
            2 => {
                input.clear();                println!("학생 제거");
                println!("이름을 입력해주세요: ");
                io::stdin().read_line(&mut input).expect("입력 오류");
                let name = input.trim().to_string();
                //빈 값이 들어왔을 때의 예외처리가 안돼있음.
                students.remove(&name);
                println!("학생 제거 완료");
            },
            3 => {
                println!("학생 검색");
                println!("이름을 입력해주세요: ");
                io::stdin().read_line(&mut input).expect("입력 오류");
                let name = input.trim().to_string();
                let find_student = students.get(&name);
                match find_student{
                    Some(std) => {
                        println!("학생 이름: {}", name);
                        for i in 0..3{
                            println!("{}과목 성적: {}", clase_name[i], std[i]);
                        }
                    }
                    None => {
                        println!("해당 학생이 존재하지 않습니다.");
                    }
                }
                println!("학생 검색 완료");
            },
            4 => {
                println!("----학생 목록----");
                for (key, value) in &students{
                    println!("{}.번 이름: {}",count, key );
                    count += 1;
                    println!("성적: {:?}", value);
                }
                count = 1;
                println!("---------------");
            },
            5 => {
                println!("프로그램을 종료합니다.");
                break;
            }
            _ => {
                println!("1~5 사이의 숫자를 입력해주세요.");
                continue;
            }
        }
    
        
    }
}
