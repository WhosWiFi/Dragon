use std::env;
use std::fs;
fn main() {
    println!("The Dragon is Born 🐉");
    println!("Select the file you would like to Compress (-c) or Decompress (-d)");
    let file = fs::read_to_string("C:/Users/Carson/Desktop/rust.txt").expect("Could not open file :(\n");
    println!("The contents of the file:\n {file}");
}
