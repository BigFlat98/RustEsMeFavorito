//모듈 선언
pub mod math{
    pub fn add(a:i32, b:i32) -> i32{ //pub -> public과 같은 개념. 즉 외부에서 접근 가능
        a + b
    }
    pub fn sub(a:i32, b:i32) -> i32{
        a - b
    }
}

pub mod my_module{
    //상수 
    pub const PI: f64 = 3.14159;

    //구조체
    pub struct Circle{
        pub radius: f64,
    }
    pub struct Rectangle{
        pub width: f64,
        pub height: f64,
    }

    //열거형
    pub enum Shape{
        Circle(Circle),
        Rectangle(Rectangle),
    }

    //함수
    pub fn area(shape: &Shape) -> f64{
        match shape{
            Shape::Circle(c) => c.radius * c.radius * PI,
            Shape::Rectangle(r) => r.width * r.height,
        }
    }

    //메서드
    impl Circle{
        pub fn new(radius: f64) -> Self{
            Self{radius}
        }
    }

    //중첩 모듈
    mod inner_module{
        pub fn say_hello(){
            println!("Hello, inner module!");
        }
    }

}



