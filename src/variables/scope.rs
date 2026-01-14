pub fn run() {
    solution1();
    solution2();
    define_x();
}

pub fn solution1() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {}", x);
}

pub fn solution2() {
    let x = define_x();
    println!("{}, world", x);
}

pub fn define_x() -> &'static str {
    "hello"
}