extern crate textstat;

use std::env;
use std::process;
use textstat::handler;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {panic!("you did not supply an argument")}
    else if args.len() > 2 {panic!("too many arguments")}

    let filename: &String = &args[1];
    println!("Searching in file {}", filename);

    if let Err(e) = textstat::handler(filename) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

