pub fn run() {
    solution1();
    solution2();
    solution3();
    solution4();
}
pub fn solution1() {
   let v = {
       let mut x = 1;
       x += 2;
       x
   };

   assert_eq!(v, 3);

   println!("Success!");
}
pub fn solution2() {
   let v = {
       let x = 1;
       x + 2
   };

   assert_eq!(v, 3);

   println!("Success!");
}

pub fn solution3() {
   let x = 3;
   let v = x;

   assert!(v == 3);

   println!("Success!");
}

pub fn solution4() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}