#[macro_use] extern crate lazy_static;

use std::env;
mod utils;
mod classify;
mod train;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    if query.eq(&String::from("-t")){
        train::train(&args[2]);
    }else if query.eq(&String::from("-c")) {
        classify::classify(&args[2], &args[3]);
    }
}











