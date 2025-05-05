pub fn largest(){
    let bigger = large(1, 2);
    let bigger_char = large('a', 'b');
    println!("{}", bigger);
    println!("{}", bigger_char);
}

fn large<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}