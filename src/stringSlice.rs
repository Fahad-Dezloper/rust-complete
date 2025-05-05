pub fn StringSlice(){
    let mut name = String::from("Hello World");
    let ans = first_word(&name);
    println!("ans is {}", ans);
}

fn first_word(str: &String) -> &str{
    let mut space_index = 0;
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index + 1;
    }
    return &str[0..space_index];
}