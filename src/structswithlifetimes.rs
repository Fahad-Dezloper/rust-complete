struct User<'a> {
    name: &'a str
}

pub fn structswithlifetimes(){
    let first_name = String::from("Fahad");
    let user = User { name: &first_name };
    println!("The name of the user is {}", user.name)
}