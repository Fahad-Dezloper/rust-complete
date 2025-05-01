pub fn run(){
    let my_string = String::from("Hello World");
    let length = get_string_len_chars(my_string);   
    println!("The number of character in string is: {}", length);
}

fn get_string_len_chars(s: String) -> usize {
    return s.chars().count();
}