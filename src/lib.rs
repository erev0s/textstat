use std::io::prelude::*;
use std::fs::File;
use std::error::Error;



pub fn handler(filename: &String) -> Result<(), Box<Error>> {
    let mut f: File = File::open(filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("{}", contents);

    Ok(())
}