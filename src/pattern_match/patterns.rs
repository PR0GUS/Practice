pub fn run() {
    solution1();
    solution2();
    solution3();
    solution4();
    solution5();
    solution6();
}

pub fn solution1() {}

fn match_number(n: i32) {
    match n {
        1 => println!("One!"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        //solution1: make 4 separate branches 2 =>, 3 =>, 4 =>, 5 =>
        6..=10 => {
            println!("match 6 -> 10")
        }
        _ => {
            println!("match -infinite -> 0 or 11 -> +infinite")
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

pub fn solution2() {
    // condition of the 2nd branch: x у 0..=5, y у {10, 20, 30}
    let p = Point { x: 3, y: 20 };
    //solution1: Point { x: 0, y: 10 } or Point { x: 5, y: 30 }

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Hello { id: i32 },
}

pub fn solution3() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range [3, 7]: {}", id)
        }
        Message::Hello { id: newid @ (10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
    //Solution1: Message::Hello { id: 10..=12 } without `@`, but then you won't print the actual value
}

pub fn solution4() {
    let num = Some(4);
    let split = 5;

    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
    //Solution1: if let Some(x) = num { if x < split { ... } else { ... } }

    println!("Success!");
}

pub fn solution5() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first, .., last) => {
            assert_eq!(first, 2);
            assert_eq!(last, 2048);
        }
    }
    //Solution1: (first, _, _, _, _, _, _, _, _, _, last)

    println!("Success!");
}

pub fn solution6() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"),
        //Solution1: match r { v => { (*v).push_str(" world!"); } }
    }
}





