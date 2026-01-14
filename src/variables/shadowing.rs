pub fn run() {
    solution1();
    solution2();
    solution3();
    solution4();
}
pub fn  solution1() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}

pub fn solution2() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    let x = 12;
    assert_eq!(x, 12);

    let x = 42;
    println!("{}", x);
}

pub fn solution3() {
    let mut x: i32 = 1;
    x = 7;
    x += 3;

    let y = 4;
    let y = "I can also be bound to text!";

    println!("Success!");
}

pub fn solution4() {
    let mut x: i32 = 1;
    x = 7;
    let mut x = x;
    x += 3;

    let y = 4;
    let y = "I can also be bound to text!";

    println!("Success!");
}
