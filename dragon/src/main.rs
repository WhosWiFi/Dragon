use std::env;
use std::fs;
use std::process::Command;
use std::process::Output;
use regex::Regex;

fn read_file(flag:&String, file_path:&String) {
    println!("The Flag Provided: {:?}", flag);
    println!("The File Path Provided: {:?}", file_path);
    let mut command_output = Command::new("pwd").output().expect("Could not execute command properly");
    println!("The Status Code is: {}", command_output.status);
    println!("The Output of the command pwd: {}", String::from_utf8_lossy(&command_output.stdout));
    println!("The Standard Error is: {}", String::from_utf8_lossy(&command_output.stderr));
    let current_directory = String::from_utf8_lossy(&command_output.stdout).trim().to_string();
    let directory_create_file = current_directory + "/";
    println!("This is the file path:\n {}", &directory_create_file);
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
