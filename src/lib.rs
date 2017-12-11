use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::iter::FromIterator;


pub fn handler(filename: &String) -> String{
    let mut f: File = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("file cannot be read");

    println!("{}", contents);

    contents
}

//hashmap to find how many times a word is re
pub fn most_used(contents: &String){

    let mut map: BTreeMap<&str, i32> = BTreeMap::new();
    for word in contents.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let mut v = Vec::from_iter(map);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    println!("{:?}",v)
}
