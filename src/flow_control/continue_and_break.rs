pub fn run() {
    solution1();
    solution2();
}

pub fn solution1() {
    let mut n = 0;
    for _ in 0..=100 {
        if n == 66 {
            break;
        }
        n += 1;
    }

    assert_eq!(n, 66);
    println!("Success!");
}

pub fn solution2() {
    let mut n = 0;
    for _ in 0..=100 {
        if n != 66 {
            n += 1;
            continue;
        }
        break;
    }
    //solution1: change the condition and do not use continue

    assert_eq!(n, 66);
    println!("Success!");
}

