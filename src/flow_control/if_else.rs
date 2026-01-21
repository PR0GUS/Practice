pub fn run() {
    solution1();
    solution2();
}

pub fn solution1() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }
}

pub fn solution2() {
    let n = 5;

    let big_n = if n < 10 && n > -10 {
        10 * n
    } else {
        n / 2
    };
    //solution1: convert n to f64 and return a fractional number

    println!("{} -> {}", n, big_n);
}
