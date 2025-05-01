pub fn run() {
    println!("{}", even(20));
}

fn even(num: i32) -> bool{
    if num % 2 == 0 {
        return true;
    }
    return false;
}