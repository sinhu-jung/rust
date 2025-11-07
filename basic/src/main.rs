fn main() {
    // 변수와 가변성
    let x = 5;
    println!("The value of x is: {}", x);

    // Rust에서 기본적으로 변수는 불변(immutable)
    // x = 6;
    // println!("The value of x is: {}", x);

    let mut y = 10; // 가변(mutable) 변수 선언
    println!("The value of y is: {}", y);
    y = 15; // 가변 변수는 값을 변경할 수 있음
    println!("The value of y is: {}", y);
    
}
