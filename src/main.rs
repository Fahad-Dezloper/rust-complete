mod is_even;
mod fibo;
mod strinlen;
mod structs;
mod structwithfn;
mod enums;
mod options;
mod readfromfile;

fn main() {
    is_even::run();
    fibo::run();
    strinlen::run();
    structs::structing();
    structwithfn::rectwithfn();
    enums::enums();
    options::options();
    readfromfile::readfromfile();

    // println!("Hello, world!");
}
