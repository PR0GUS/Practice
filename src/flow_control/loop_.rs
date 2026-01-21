pub fn run() {
    solution1();
    solution2();
    solution3();
}

pub fn solution1() {
    let mut count = 0u32;

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }

        println!("{}", count);

        if count == 5 {
            break;
        }
    }
    //solution1: if/else without continue

    assert_eq!(count, 5);
    println!("Success!");
}

pub fn solution2() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    //solution1: count after loop

    assert_eq!(result, 20);
    println!("Success!");
}

pub fn solution3() {
    let mut count = 0;

    'outer: loop {
        'inner1: loop {
            if count >= 20 {
                break 'inner1;
            }
            count += 2;
        }

        count += 5;

        loop {
            if count >= 30 {
                break 'outer;
            }
            continue 'outer;
        }
    }


    assert!(count == 30);
    println!("Success!");
}

 //solution2:
    // Solution: the same logic can be done without labels via while + break/continue
    /*
    let mut count = 0;
    loop {
        while count < 20 {
            count += 2;
        }
        count += 5;
        if count >= 30 { break; }
        // otherwise we just continue the loop (it's like continue outer)
    }
    assert_eq!(count, 30);
    */

    //Solution3:
    // Solution: you can simply calculate the final value (here always 30) without loops
    /*
    let mut count = 0;
    // count after inner1: 20, then +5 => 25, another outer iteration: inner1 nothing, +5 => 30
    count = 30;
    assert_eq!(count, 30);
    */
