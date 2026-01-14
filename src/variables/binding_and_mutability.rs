pub fn run() {
    solution1();
    solution2();
}
pub fn  solution1() {
    let x: i32 = 5;
    let y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

pub fn solution2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}