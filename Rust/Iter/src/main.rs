fn main() {
    let mut xy = vec![vec![0;9];9];// 여기선 쓸 이유가 없긴 하네. push / pop 메서드로 추가 삭제 가능
    let mut newArray = [9][9];
    for i in 1..newArray.len(){
        for j in 1..newArray.len(){
            newArray[i][j] = i * j;
        }
    }
    for i in 1..newArray.len(){
        for j in 1..newArray.len(){
            println!("{} * {} = {}", i, j, newArray[i][j]);
        }
    }
    
}
