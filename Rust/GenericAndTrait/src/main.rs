fn main() {
    println!("Hello, world!");

    let number:Vec<i32> = vec![1,3,-32,78,190,-1,-90];
    let result = largest_i32(&number);
    println!("The largest number is {}", result);
    println!("The largest number is {}", result);
    println!("The largest number is {}", result);

    

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_att(&char_list);
    println!("The largest char is {}", result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn largest_att<T: PartialOrd + Copy>(list: &[T]) -> T { //PartralOrd는 비교가 가능한 타입을 의미. Copy는 copy가 가능한 타입만 사용 가능하도록 제한하는 트레이트. 만약 copy를 안쓰면 참조형을 사용해야 함.
    //제네릭을 사용하면 함수에 여러 타입을 사용해 타입별로 동일한 함수를 정의할 필요가 없어짐.
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

