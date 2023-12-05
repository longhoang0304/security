use std::io;
use std::io::Write;
use crate::pe::readers;

mod pe;
mod crypto;
mod helpers;

fn main() {
    let mut path = String::new();

    print!("{}", "PE Path: ");
    io::stdout().flush().expect("Flushed");
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to pe file");

    println!("Reading PE from: {}", path);

    let path = path.strip_suffix("\r\n").unwrap();

    match readers::read_and_parse(&path) {
        Ok(data) => println!("{}", data),
        Err(e) => eprintln!("{}", e)
    };
}
