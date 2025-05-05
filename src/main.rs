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
mod vector;
mod hashmaps;
mod hashmapQues;
mod iters;
mod intoiters;
mod stringSlice;

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
    vector::vec();
    hashmaps::hash();
    hashmapQues::hash_map_que();
    iters::iters();
    intoiters::intoiters();
    stringSlice::StringSlice();

    // println!("Hello, world!");
}
