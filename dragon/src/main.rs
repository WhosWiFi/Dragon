use std::env;
use std::fs;
use std::io::Read;

fn read_file(flag:&String, file_path:&String) {
    println!("The Flag Provided: {:?}", flag);
    println!("The File Path Provided: {:?}", file_path);
    let mut file = fs::File::open(file_path).expect("Could not open file :(\n");
}

fn main() {
    println!("The Dragon is Born 🐉");
    println!("How to use Dragon -> dragon [flags: -c (compress), -d (decompress)] 'path_to_file']");
    let cli_args: Vec<String> = env::args().collect();
    println!("{:?}", cli_args);
    let flag: &String = &cli_args[1];
    let file_path: &String = &cli_args[2];
    read_file(flag, file_path);
}
