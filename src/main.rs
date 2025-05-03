mod is_even;
mod fibo;
mod strinlen;
mod structs;
mod structwithfn;
mod enums;
mod options;
mod readfromfile;
mod datetime;
mod owner;
mod borrow;

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
    owner::owner();
    borrow::borrow();

    // println!("Hello, world!");
}
