pub fn run() {
    solution1();
    solution2();
    solution3();
}

pub fn solution1() {
    for n in 1..100 {
        if n == 100 {
            panic!("NEVER LET THIS RUN")
        }
    }
    println!("Success!");
}

pub fn solution2() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {
        println!("Hello, {}", name);
    }
    //solution1: names.clone(), but it is less efficient

    println!("{:?}", names);

    let numbers = [1, 2, 3];
    for n in numbers {
        println!("{}", n);
    }
    //solution2 : for n in &numbers

    println!("{:?}", numbers);
}

pub fn solution3() {
    let a = [4, 3, 2, 1];

    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
    //solution1: calculate the index manually
}
