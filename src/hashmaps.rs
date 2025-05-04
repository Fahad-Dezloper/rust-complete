use std::collections::HashMap;

pub fn hash(){
    let mut users = HashMap::new();

    users.insert(String::from("Fahad"),  22);
    users.insert(String::from("Raman"),  22);

    // hashmaps exampale
    // {
    //     fahad: 22
    //     ramad: 21
    // }

    let first_user_age = users.get("Fahad");
    match first_user_age {
        Some(age) => println!("aget is {}", age),
        None => println!("User not found in the db"),
    }
}