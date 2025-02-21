use chrono::{Utc, Local};
use std::ops::Add;

fn main(){
    print!("{}", sum(1,2));
}

fn sum<T: Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}


// let utc = Utc::now();
// let local_time = Local::now();
// print!("{}", utc);
// print!("{}", local_time);
