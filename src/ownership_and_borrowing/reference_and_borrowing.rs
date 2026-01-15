pub fn run() {
    solution1();
    solution2();
    solution3();
    solution4();
    solution5();
    solution6();
    solution7();
    solution8();
    solution9();
    solution10();
    solution11();
}

pub fn solution1() {
    let x = 5;
    let p = &x;

    println!("the memory address of x is {:p}", p);
}

pub fn solution2() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);

    println!("Success!");
}

pub fn solution3() {
    let s = String::from("hello, ");

    borrow_object_immutable(&s);

    println!("Success!");
}

// separated functions for immutable/mutable borrow
fn borrow_object_immutable(s: &String) {}

pub fn solution4() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}

pub fn solution5() {
    let mut s = String::from("hello, ");

    let p = &mut s;

    p.push_str("world");
    println!("Success!");
}
/*Solution 5.1
pub fn solution5() {
  let mut s = String::from("hello, ");

  let p: &mut String = &mut s;

  p.push_str("world");
   println!("Success!");
 }
*/

pub fn solution6() {
    let c = '中';
    let r1 = &c;

    let ref r2 = c; 

    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}
/*Solution 6.1
pub fn solution6() {
    let c = '中';
    let r1 = &c;

    let r2 = &c;

    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}
 Solution 6.2
pub fn solution6() {
    let c = '中';
    let r1 = &c;

    let r2: &char = &c;

    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}
*/

pub fn solution7() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    println!("{}", r1);
    println!("Success!");
}

pub fn solution8() {
    let mut s = String::from("hello, ");
    borrow_object_mutable(&mut s);

    println!("Success!");
}

// function for mutable borrow
fn borrow_object_mutable(s: &mut String) {}

pub fn solution9() {
    let mut s = String::from("hello, ");
    borrow_object_immutable(&s);
    s.push_str("world");
    println!("Success!");
}

pub fn solution10() {
    let mut s = String::from("hello, ");
   
    let r1 = &mut s;
    r1.push_str("world");
    // println!("{}", r1);

    let r2 = &mut s;
    r2.push_str("!");
    
    println!("{}", r2);
}

pub fn solution11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    // let r2 = &mut s; This line causes compiler error E0499

    // use r1
    println!("{}", r1);

}

/*Solution 11.1
pub fn solution11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    //use r2
    r2.push_str("!");
}
*/

// helper function for ref example
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
