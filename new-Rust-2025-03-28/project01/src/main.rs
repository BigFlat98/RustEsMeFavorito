fn main() {
    let c = 30i32;
    let d = 30_i32;
    println!("{}",c);
    println!("{}",d);
    println!("{}",add(c,d));

    let three = 0b11; //2진
    let thirty =0o36; //8진
    let three_hundred = 0x12C; //16진

    println!("{}",three);
    println!("{}",thirty);
    println!("{}",three_hundred);
    
    
}

fn add(a:i32,b:i32)->i32{
    a+b //;을 붙이면 () 유닛 타입을 반환.
}
