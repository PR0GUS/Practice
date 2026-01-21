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

enum Direction {
    East,
    West,
    North,
    South,
}

pub fn solution1() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => { 
            println!("South or North");
        },
        _ => println!("West"), 

}
}

pub fn solution2() {
    let boolean = true;

    let binary = match boolean { 
        true => 1,
        false => 0,
    };
    //solution1: `let binary = if boolean { 1 } else { 0 };`

    assert_eq!(binary, 1);

    println!("Success!");
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn solution3() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }

    println!("Success!");
}

fn show_message(msg: Message) {
    match msg {
        Message::Move { x: a, y: b } => { // unpack
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        },
        _ => println!("no data in these variants"), // other cases
    }
    //solution1: could add separate branches for Quit і Write
}

pub fn solution4() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'A'..='Z' | 'a'..='z')); // букви
    }
    //solution1: use `.is_ascii_alphabetic()`

    println!("Success!");
}

enum MyEnum {
    Foo,
    Bar
}

pub fn solution5() {
    let mut count = 0;
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    for e in v {
        if matches!(e, MyEnum::Foo) { //if let or matches!
            count += 1;
        }
        //solution1: use `if let MyEnum::Foo = e`
    }

    assert_eq!(count, 2);

    println!("Success!");
}

pub fn solution6() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);

        println!("Success!");
    }
    //Solution1: the entire match block could be improved
}

enum Foo {
    Bar(u8)
}

pub fn solution7() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a { // if let for unpack
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
    //solution1: full match
}

enum Foo2 {
    Bar,
    Baz,
    Qux(u32)
}

pub fn solution8() {
    let a = Foo2::Qux(10);

    match a { //all cases through match
        Foo2::Bar => println!("match foo::bar"),
        Foo2::Baz => println!("match foo::baz"),
        Foo2::Qux(_) => println!("match others"),
    }
    //Solution1: you could have left the initial if let option
}

pub fn solution9() {
    let age = Some(30);
    if let Some(age) = age { // variable shadow
       assert_eq!(age, 30); // here age is already a number
    }

    match age {
        Some(age) => println!("age is a new variable, it's value is {}", age),
        _ => ()
    }
    //solution1: you could use `.map()` to process Option
}



