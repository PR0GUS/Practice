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
}

pub fn solution1() {
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}", x, y);
}
/*Solution1.1:
pub fn solution1() {
    let x = String::from("Hello world");
    let y = &x;
    println!("{}, {}", x, y);
}
*/

pub fn solution2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

/*Solution2.1:
pub fn solution2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership_borrow(s1);
    println!("{}", s2);
}

fn take_ownership_borrow(s: String) -> String {
    println!("{}", &s);
    s
}*/

pub fn solution3() {
    let s = give_ownership_clone();
    println!("{}", s);
}

fn give_ownership_clone() -> String {
    let s = String::from("Hello world");
    let _v = s.clone().into_bytes();
    s
}
/*Solution3.1:
pub fn solution3() {
let s = give_ownership_restore();
    println!("{}", s);
}

fn give_ownership_restore() -> String {
    let s = String::from("Hello world");
    let v = s.into_bytes();
    String::from_utf8(v).unwrap()
}
*/

pub fn solution4() {
    let s = String::from("Hello World");
    print_str_ref(&s);
    println!("{}", s);
}

fn print_str_ref(s: &String) {
    println!("{}", s);
}
/*Solution4.1
pub fn solution4() {
    let s = String::from("Hello World");
    print_str_str(&s);
    println!("{}", s);
}

fn print_str_str(s: &str) {
    println!("{}", s);
}
*/
pub fn solution5() {
    let x = (1, 2, (), "hello".to_string());
    let (a, b, c, ref d) = x;
    let y = (a, b, c, d.clone());
    println!("{:?}, {:?}", x, y);
}
/*Solution5.1
pub fn solution5 () {
    let x = (1, 2, (), "hello".to_string());
    let y = (&x.0, &x.1, &x.2, &x.3);
    println!("{:?}, {:?}", x, y);
}
*/
pub fn solution6() {
    let s = String::from("Hello ");
    let mut s1 = s;
    s1.push_str("World!");
    println!("Success!");
}

pub fn solution7() {
    let x = Box::new(5);
    let mut y = Box::new(4);
    assert_eq!(*x, 5);
    println!("Success!");
}
/*Solution7.1
pub fn solution7() {
    let x = Box::new(5);
    let y = &x;
    assert_eq!(*x, 5);
    println!("Success!");
}
*/

pub fn solution8() {
    let t = (String::from("hello"), String::from("world"));
    let _s = &t.0;
    println!("{:?}", t);
}

pub fn solution9() {
    let t = (String::from("hello"), String::from("world"));
    let (s1, s2) = (&t.0, &t.1);
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}
/*Solution9.1
pub fn solution9() {
    let t = (String::from("hello"), String::from("world"));
    let (ref s1, ref s2) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t);
}
*/