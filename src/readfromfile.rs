use std::fs::read_to_string;

pub fn readfromfile(){
    let result = read_to_string("a.txt");
    match result {
        Ok(data) => println!("{}", data),
        Err(err) => println!("Error while reading the file")
    }
}