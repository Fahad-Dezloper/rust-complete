mod is_even;
mod fibo;
mod strinlen;
mod structs;
mod structwithfn;
mod enums;
mod options;
mod readfromfile;
mod datetime;

fn main() {
    is_even::run();
    fibo::run();
    strinlen::run();
    structs::structing();
    structwithfn::rectwithfn();
    enums::enums();
    options::options();
    readfromfile::readfromfile();
    datetime::dateandtime();

    // println!("Hello, world!");
}
