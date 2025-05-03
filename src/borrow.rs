pub fn borrow(){
    let s1 = String::from("Fahad");
    do_something(&s1);
    println!("string after fn {}", s1);
}

fn do_something(s2: &String){
    println!("inside fn {}", s2);
}