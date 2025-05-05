pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

pub fn tra() {
    let user = User {
        name: String::from("Fahad"),
        age: 21
    };

    println!("{}", user.summarize());
}