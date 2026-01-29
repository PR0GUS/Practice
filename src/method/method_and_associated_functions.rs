pub fn run() {
    solution1();
    solution2();
    solution3();
    solution4();
    solution5();
    solution6();
}

// Rectangle.area()
pub fn solution1() {
    struct Rectangle1 {
        width: u32,
        height: u32,
    }

    impl Rectangle1 {
        //Solution1: &self borrow
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect1 = Rectangle1 { width: 30, height: 50 };
    assert_eq!(rect1.area(), 1500);
    println!("Success!");
}

// show_state(&self) (do not take ownership)
pub fn solution2() {
    #[derive(Debug)]
    struct TrafficLight1 {
        color: String,
    }

    impl TrafficLight1 {
        //Solution1: use &self to borrow
        pub fn show_state(&self) {
            println!("the current state is {}", self.color);
        }
    }

    let light = TrafficLight1 {
        color: "red".to_owned(),
    };

    light.show_state();
    println!("{:?}", light);
}

// Self у show_state + change_state(&mut self)
pub fn solution3() {
    struct TrafficLight2 {
        color: String,
    }

    impl TrafficLight2 {
        //Solution1: Self alias in signature
        pub fn show_state(self: &Self) {
            println!("the current state is {}", self.color);
        }

        //Solution1: &mut self (no Self variants)
        pub fn change_state(&mut self) {
            self.color = "green".to_string()
        }
    }

    let mut t = TrafficLight2 {
        color: "red".to_string(),
    };
    t.show_state();
    t.change_state();
    t.show_state();
    println!("Success!");
}

// Associated function new() -> Self
pub fn solution4() {
    #[derive(Debug)]
    struct TrafficLight3 {
        color: String,
    }

    impl TrafficLight3 {
        //Solution1: associated function returning Self
        pub fn new() -> Self {
            Self {
                color: "red".to_string(),
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }

    let light = TrafficLight3::new();
    assert_eq!(light.get_state(), "red");
    println!("Success!");
}

// Multiple impl blocks
pub fn solution5() {
    struct Rectangle2 {
        width: u32,
        height: u32,
    }

    //Solution1: split methods into multiple impl blocks
    impl Rectangle2 {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    impl Rectangle2 {
        fn can_hold(&self, other: &Rectangle2) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let r1 = Rectangle2 { width: 5, height: 6 };
    let r2 = Rectangle2 { width: 4, height: 5 };

    assert_eq!(r1.area(), 30);
    assert!(r1.can_hold(&r2));
    println!("Success!");
}

// Methods for enum
pub fn solution6() {
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }

    //Solution1: implement method with match
    impl TrafficLightColor {
        fn color(&self) -> &'static str {
            match self {
                TrafficLightColor::Red => "red",
                TrafficLightColor::Yellow => "yellow",
                TrafficLightColor::Green => "green",
            }
        }
    }

    let c = TrafficLightColor::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{:?}", c);
}
