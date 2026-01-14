pub fn run() {
    solution1();
    solution2();
}

pub fn solution1() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

pub fn solution2() {
    let (x, y) = (1, 2);
    let x = x + 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}