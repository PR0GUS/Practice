pub fn run() {
    solution1();
    solution2();
    solution3();
    solution4();
    solution5();
    solution6();
}

pub fn solution1() {
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn solution2() {
    print();
}

pub fn print() {
    println!("Success!");
}

pub fn solution3() {
    if false {
        never_return(); 
    }
    println!("Success!");
    //alternative: return; before never_return()
}

// Solution1: infinite loop
pub fn never_return() -> ! {
    panic!("This function never returns");
    //alternative: unreachable!()
    //alternative: loop {}
}


pub fn solution4() {
    never_return_loop();
}

//Solution1: infinite loop
fn never_return_loop() -> ! {
    loop {}
}

pub fn solution5() {
    println!("Success!");
}

pub fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42),
        _ => never_return_fn(),
    }
}

// Solution1: use loop {}
fn never_return_fn() -> ! {
    panic!("No value");
}

pub fn solution6() {
    let b = false;

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for false, but we can panic");
        }
    };
}
