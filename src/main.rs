mod is_even;
mod fibo;
mod strinlen;
mod structs;
mod structwithfn;
mod enums;

fn main() {
    is_even::run();
    fibo::run();
    strinlen::run();
    structs::structing();
    structwithfn::rectwithfn();
    enums::enums();

    // println!("Hello, world!");
}
