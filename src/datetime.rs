use chrono::{Local, Utc};

pub fn dateandtime(){
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);
}