struct Rect {
    width: i32,
    height: i32
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }
    fn perimeter(&self) -> i32 {
        2 * (self.width * self.height)
    }
}

pub fn rectwithfn() {
    let rect1 = Rect {
        width: 10,
        height: 20
    };

    println!("area is {}", rect1.area());
    println!("perimeter is {}", rect1.perimeter());
}