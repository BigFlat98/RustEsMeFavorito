use std::io;

fn main() {
    let number = 6;
    if number < 5 {
        println!("number는 5보다 작다.");
    } else if number > 5 {
        println!("number는 5보다 크다.");
    } else {
        println!("number는 5와 같다.");
    }

    //loop
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if(counter == 10){
            break counter + 5; //break 문은 반복문을 종료하고 값을 반환합니다. -> 다른 언어들과의 차별점
        }
    };
    println!("반복문의 결과: {}", result);

    //while
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    //for
    for number in 1..4{
        println!("{}", number);
    }

    //for 문을 이용한 배열 순회
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("요소: {}", element);
    }

    //for 문을 이용한 배열 순회(역순)
    for number in (1..4).rev() {
        println!("{}", number);
    }

    //match
    let dice_roll = 6;
    match dice_roll{
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),
        6 => println!("6"),
        _ => println!("눈금이 없습니다."),
    }

    

    //소유권
    let s1 = String::from("muy cansado");
    let s2 = s1; //s1의 소유권이 s2로 이전됩니다.
    //println!("{}", s1); //s1의 소유권이 이전되었기 때문에 오류가 발생합니다.
    println!("{}", s2);

    //clone
    let s3 = String::from("muy contento");
    let s4 = s3.clone(); //데이터를 복사하여 새로운 변수에 할당합니다.
    println!("{}", s3);
    println!("{}", s4);

    //참조와 대여
    let s5 = String::from("muy triste");
    let len = calculate_length(&s5); //참조를 이용하여 문자열의 길이를 계산합니다. &는 불변 참조, &mut는 가변 참조
    println!("문자열의 길이: {}", len);

    //가변 참조
    let mut s6 = String::from("muy cansado");
    let r1 = &mut s6; //가변 참조
    println!("{}", r1);
    r1.push_str(" y enojada"); //push_str은 문자열을 추가합니다.
    println!("{}", r1);
    println!("{}", s6);


    //실습 성적 관리 프로그램
    let mut count = 0;
    let mut sum = 0;
    while true{
        count += 1;
        let mut score = String::new();
        println!("점수를 입력해주세요.");
        io::stdin().read_line(&mut score).expect("입력 오류");
        let converted_score: i32 = match score.trim().parse(){
            Ok(num) => num, //여기에서는 변환한 값의 범위를 확인해서 여기서 잘못된 입력을 확인해 줄 수 있음.
            Err(_) => {
                println!("숫자를 입력해주세요.");
                0
            },
        };

        if converted_score >= 90{
            sum += converted_score;
            println!("A");
        } else if converted_score >= 80{
            sum += converted_score;
            println!("B");
        } else if converted_score >= 70{
            sum += converted_score;
            println!("C");
        } else if converted_score >= 60{
            sum += converted_score;
            println!("D");
        } else if converted_score >= 0{
            sum += converted_score;
            println!("F");
        } else{
            println!("잘못된 입력입니다.");
        }
        println!("추가 진행을 원하시면 1, 종료를 원하시면 0을 입력해주세요.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("입력 오류");
        let input: i32 = input.trim().parse().expect("숫자를 입력해주세요.");
        if input == 0 {
            println!("평균: {}", (sum / count));
            break;
        }
        else if input == 1 {
            continue;
        }
        else{
            println!("잘못된 입력입니다.");
        }
        
    }
}
fn calculate_length(s: &String) -> usize { //usize는 문자열의 길이를 나타내는 타입입니다.
    s.len()
}

//match 문을 활용한 성적 계산 함수 
fn calculate_grade(score: i32) -> char{
    match score{
        90..=100 => 'A', //90~100을 90..=100으로 표현 가능
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => 'X',
    }
}


//실습에서 리펙토링 할만한건 loop문과 match 문을 적극적으로 활용할 필요가 있고 
//되도록이면 메서드를 만들어 사용하는 습관을 들일 필요가 있을 것 같음.