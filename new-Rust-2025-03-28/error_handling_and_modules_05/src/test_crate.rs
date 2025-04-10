pub mod test_module{//상위 스코프가 pub이어도 내부에 있는 변수나 함수 등 모든 내용은 기본적으로 private임.
    pub fn say_hello(){
        println!("Hello, test_module!");
    }
}

pub mod test_module2{
    pub fn say_hello(){
        println!("Hello, test_module2!");
    }
}
