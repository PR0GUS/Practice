use std::mem::size_of_val;

pub fn run() {
    solution1();
    solution2();
    solution3();
    solution4();
    solution5();
    solution6();
}

pub fn solution1() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

pub fn solution2() {
    let c1 = '中';
    print_char(c1);
}

pub fn print_char(c: char) {
    println!("{}", c);
}

pub fn solution3() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

pub fn solution4() {
    let f = false;
    let t = true && false;
    assert_eq!(t, f);

    println!("Success!");
}


pub fn solution5() {
    let _v: () = ();

    let v = implicitly_ret_unit();
    assert_eq!(v, ());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}


pub fn solution6() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}