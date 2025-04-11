//트레이트 정의
//트레이트는 자바의 인터페이스과 유사함.(같은 개념 같음)
trait Animal{

    //추상 메서드들 -> 트레이트를 구현하는 구조체들은 반드시 오버라이딩 해야함.
    fn run(&mut self)->i32;
    fn sound(&self)->String;

    //기본 구현이 돼있는 메서드들
    fn eat(&self){
        println!("eating now");
    }
    fn sleep(&self){
        println!("sleeping now");
    }
}

struct Person{
    name:String,
    age:u8,
    speed:i32,
}

struct Dog{
    name:String,
    age:u8,
    speed:i32,
}

//Person 구조체가 Animal 트레이트를 구현하는 것을 보여줌
impl Animal for Person{
    //Person 구조체의 메서드를 오버라이딩
    fn run(&mut self)->i32{
        self.speed+=5;
        println!("{} is running",self.name);
        self.speed
    }
    fn sound(&self)->String{
        "Hello".to_string()
    }
    fn eat(&self){
        println!("{} is eating",self.name);
    }
    fn sleep(&self){
        println!("{} is sleeping",self.name);
    }
}

impl Animal for Dog{
    fn run(&mut self)->i32{
        self.speed+=10;
        println!("{} is running",self.name);
        self.speed
    }
    fn eat(&self){
        println!("{} is eating",self.name);
    }
    fn sleep(&self){
        println!("{} is sleeping",self.name);
    }
    fn sound(&self)->String{
        "Woof".to_string()
    }
}

impl Person{
    fn new(name:String,age:u8,speed:i32)->Self{
        Self{name,age,speed}
    }
}

//트레이트 바운드 : 제네릭 타입에 제약을 추가하는 방법
//T는 어떤 타입이든 될 수 있지만, 반드시 Animal 트레이트를 구현해야 함.
//따라서 run, eat, sleep, sound 메서드가 있다는 것을 보장
//-> 이렇게 되면 파라미터로 들어오는 객체는 animal을 구현해야 한다는 것을 컴파일러가 인지할 수 있음
fn print_animal_info<T:Animal>(mut animal:T){
    animal.run();
    animal.eat();
    animal.sleep();
}







//제네릭
//제네릭이 없는 경우
fn print_integer(n: i32){
    println!("n: {}",n);
}
fn print_float(f:f32){
    println!("f: {}",f);
}
fn print_string(s:String){
    println!("s: {}",s);
}

//제네릭을 사용했을 때
fn print_any<T>(value:T) where T: std::fmt::Display{//파라미터를 제네릭으로 지정한 메서드에서 파라미터 출력을 위해서는 Display트레이트가 필요함
    println!("value: {}",value);
}



//단일 타입 파라미터를 갖는 제네릭 구조체
struct Point<T>{
    x:T,
    y:T,
}

//여러 타입 파라미터를 갖는 제네릭 구조체
struct Pair<T,U>{
    first:T,
    second:U,
}

//제네릭 메서드 구현
impl<T> Point<T>{
    fn new(x:T,y:T)->Self{
        Self{x,y}
    }

    fn get_x(&self)->&T{
        &self.x
    }

    fn get_y(&self)->&T{
        &self.y
    }
}
impl Point<f64>{ //타입이 f64인 구조체에 대한 객체만 갖는 메서드. T타입 메서드도 당연히 사용 가능
    fn distance_from_origin(&self)->f64{
        (self.x.powi(2)+self.y.powi(2)).sqrt()//powi()는 제곱을 구하는 메서드, sqrt()는 제곱근을 구하는 메서드
    }
}
impl<T,U> Pair<T,U>{
    fn new(first:T,second:U)->Self{
        Self{first,second}
    }

    fn get_first(&self)->&T{
        &self.first
    }
}



//제네릭 열거형
enum Option<Type>{
    Some(Type),
    None,
}

impl<Type> Option<Type>{
    fn new(value:Type)->Self{
        Self::Some(value)
    }

    fn is_some(&self)->bool{
        matches!(self,Self::Some(_))
    }
    
    
}
enum Result<Type,Error>{
    Ok(Type),
    Err(Error),
}

//사용자 정의 제네릭 열거형
enum Either<Type1,Type2>{
    Left(Type1),
    Right(Type2),
}




//여러 다중 트레이트 바운드
trait Printable{
    fn print(&self);
}
trait Sizeable{
    fn get_size(&self)->usize;
}
//+를 사용해 여러 트레이트 바운드 지정
fn print_size<T:Printable+Sizeable>(item:T){
    item.print();
    println!("size: {}",item.get_size());
}
struct Document{
    content:String,
}
impl Printable for Document{
    fn print(&self){
        println!("Document content: {}",self.content);
    }
}
impl Sizeable for Document{
    fn get_size(&self)->usize{
        self.content.len()
    }
}


//트레이트 바운드가 복잡할 때 where 절 사용
fn complex_function<Type,Unit>(t:Type,u:Unit)->i32
where Type: Printable+Sizeable,Unit: Clone + std::fmt::Debug,
{
    t.print();
    println!("T의 크기: {}",t.get_size());
    println!("Unit: {:?}",u);
    0
}

// where 절을 사용하지 않은 경우 (복잡해 보임)
// fn complex_function<T: Printable + Sizeable, U: Clone + std::fmt::Debug>(t: T, u: U) -> i32 {
//     // ... 동일한 구현 ...
// }



//실습
trait Shape{
    fn area(&self)->f64;
    fn show_name(&self)->String;
}

struct Circle{
    name:String,
    radius:f64,
}

struct Rectangle{
    name:String,
    width:f64,
    height:f64,
}


impl Shape for Circle{
    fn area(&self)->f64{
        self.radius.powi(2)*(3.14)
    }
    fn show_name(&self)->String{
        self.name.clone()
    }
}

impl Shape for Rectangle{
    fn area(&self)->f64{
        self.width*self.height
    }
    fn show_name(&self)->String{
        self.name.clone()
    }
}

impl Circle{
    fn new(name:String,radius:f64)->Self{
        Self{name,radius}
    }
}

impl Rectangle{
    fn new(name:String,width:f64,height:f64)->Self{
        Self{name,width,height}
    }
}

fn print_area<T:Shape>(shape:T){
    println!("{}의 면적: {}",shape.show_name(),shape.area());
}






fn main() {
    //트레이트
    let mut person=Person::new("John".to_string(),20,10);
    person.run();
    person.eat();
    person.sleep();

    println!("person speed: {}",person.speed);







    //제네릭
    //제네릭이 없는 경우
    print_integer(10);
    print_float(10.0);
    print_string("Hello".to_string());

    //제네릭을 사용했을 때
    print_any(10);
    print_any(10.0);
    print_any("Hello".to_string());

    //단일 타입 파라미터를 갖는 제네릭 구조체
    let p1:Point<i32>=Point{x:1,y:2};
    let p2:Point<f64>=Point{x:1.0,y:2.0};

    //여러 타입 파라미터를 갖는 제네릭 구조체
    let p:Pair<i32,String>=Pair{first:1,second:"Hello".to_string()};



    

    //실습
    let circle = Circle::new("Circle1".to_string(),5.0);
    let rectangle = Rectangle::new("Rectangle1".to_string(),10.0,5.0);

    print_area(circle);
    print_area(rectangle);
}
