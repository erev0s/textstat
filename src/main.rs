extern crate textstat;

use std::env;
use textstat::most_used;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {panic!("you did not supply an argument")}
    else if args.len() > 2 {panic!("too many arguments")}

    let filename: &String = &args[1];
    println!("Searching in file {}", filename);

    let contents: String = textstat::handler(filename);

    most_used(&contents);

}


