use std::env;
use std::fs;
use std::io::Read;
fn main() {
    println!("The Dragon is Born 🐉");
    println!("How to use Dragon -> dragon [flags: -c (compress), -d (decompress)] 'path_to_file']");
    let cli_args: Vec<String> = env::args().collect();
    println!("{:?}", cli_args);
    let flag = &cli_args[1];
    let file_path = &cli_args[2];
    let mut file = fs::File::open(file_path).expect("Could not open file :(\n");
    println!("The Flag Provided: {:?}", flag);
    println!("The File Path Provided: {:?}", file_path);
    
}
