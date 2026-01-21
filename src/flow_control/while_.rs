pub fn run() {
    solution1();
}
pub fn solution1() {
    let mut n = 1;

    while n <= 10 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }
    //solution1: for n in 1..=10
}

