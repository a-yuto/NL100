use std::fs::File;
use std::io::prelude::*;
extern crate reqwest;

struct Article {
    text: String,
    title: String,
}

fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}
fn main() -> std::io::Result<()>{

    let mut content = match File::open("jawiki-country.json.gz") {
        Ok(file) => {file}
        Err(why) => {panic!(why.to_string())}
    };
    println!("{:?}",content);
    Ok(())
}
