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
    let mut y: u32 = 5;

    y = x;
    
    let z = 10; 

    println!("Success!");
}

pub fn solution2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

pub fn solution3() {
    let x = 5;

    //Solution1: let x: u32 = 5
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!");
}

//One type_of function for all
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

pub fn solution4() {
    
    //Solution1: use literal values
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

pub fn solution5() {
   //Solution1: casting to larger type
   let v1 = 251_u8.wrapping_add(8);
   let v2 = i8::checked_add(120, 7).unwrap();
   println!("{},{}", v1, v2);
}

pub fn solution6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;

    //Solution1: recompute manually
    assert!(v == 1595);

    println!("Success!");
}

pub fn solution7() {
    let x = 1_000.000_1;
    let y: f32 = 0.12;
    let z = 0.01_f64;

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

pub fn solution8() {

    //Solution1: use f32 instead of f64
    let diff: f64 = 0.1 + 0.2 - 0.3;
    assert!(diff.abs() < 1e-10);

    println!("Success!");
}

pub fn solution9() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    //Solution1: change range
    assert!(sum == -5);

    for c in 97u8..=122 {
        println!("{}", c as char);
    }
}

use std::ops::{Range, RangeInclusive};

pub fn solution10() {
    
    //Solution1: exclusive range
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

pub fn solution11() {
    assert!(1u32 + 2 == 3);

    assert!(1i32 - 2 == -1);
    assert!(1u8.wrapping_sub(2) == 255); 
    
    assert!(3 * 50 == 150);

    //Solution1: cast to f32
    let diff: f64 = 9.6 / 3.2 - 3.0;
    assert!(diff.abs() < 1e-10);

    assert!(24 % 5 == 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
