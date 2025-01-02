use std::env;
use std::fs;
use std::process::Command;
use std::str;
use regex::Regex;
use eframe::egui;
use rfd::FileDialog;
use aho_corasick::AhoCorasick;


fn read_file(flag:&String, file_path:&String) {
    let command_output = Command::new("pwd").output().expect("Could not execute command properly");
    let current_directory = String::from_utf8_lossy(&command_output.stdout).trim().to_string();
    let directory_create_file = current_directory + "/";
    println!("This is the file path:\n {}", &directory_create_file);

    let re = Regex::new(r"[^/]+$").unwrap();
    let file_name = if let Some(caps) = re.captures(&file_path) {
        caps.get(0).map_or("", |m| m.as_str())
    } else {
        println!("No match found");
        "NO FILE"
    };
    println!("The file name is: {}", file_name);
}

fn compress_text(data: Vec<u8>) -> String {
    println!("Dragon Processed {} bytes", data.len());
    let data_byte_size = data.len();

    let patterns = &[
        "the",
        "and",
        "all",
        "ight",
        "tion",
        "ment",
        "ness",
        "ship",
        "able",
        "ance",
        "ence",
        "ture",
        "ward",
        "ing",
        "ion",
        "ate",
        "ive",
        "ize",
        "ful",
        "ous",
        "est"
    ];

    let replace_with = &[
        "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", 
        "\x09", "\x0A", "\x0B", "\x0C", "\x0D", "\x0E", "\x0F",
        "\x10", "\x11", "\x12", "\x13", "\x14", "\x15"
    ];

    let data_as_bytes = String::from_utf8(data).expect("Error");

    // Build Aho-Corasick automaton
    let ahocora = AhoCorasick::new(patterns).unwrap();
    let compressed_data = ahocora.replace_all(&data_as_bytes, replace_with);
    
    println!("Dragon compressed the file from {} bytes to {} bytes", data_byte_size, compressed_data.len());
    println!("Contents of file:\n {}", compressed_data);

    compressed_data

}

fn main() {
    println!("The Dragon is Born 🐉");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Dragon", native_options, Box::new(|cc| Ok(Box::new(Dragon::new(cc)))));
    println!("How to use Dragon -> dragon [flags: -c (compress), -d (decompress)] 'path_to_file']");
    let cli_args: Vec<String> = env::args().collect();
    println!("{:?}", cli_args);
    let flag: &String = &cli_args[1];
    let file_path: &String = &cli_args[2];
    read_file(flag, file_path);
}

#[derive(Default)]
struct Dragon {}

impl Dragon {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn get_file(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Compress File").clicked() {
                println!("Compress File Button Clicked");
                let file = FileDialog::new()
                    .add_filter("text files", &["txt"])
                    .set_directory("/")
                    .pick_file();
                
                if let Some(file_path) = file {
                    let data = fs::read(file_path).expect("Error occured reading the file");
                    let compressed_data = compress_text(data);

                    // Get the file name (everything after last '/')
                    let re_filename = Regex::new(r"[^/]+$").unwrap();
                    let file_name = re_filename.find(file_path.to_str().unwrap())
                        .map_or("", |m| m.as_str());
                    let dragon_file = format!("!{}.dragon", file_name);

                    // Get everything up to the last '/'
                    let re = Regex::new(r"^.*(?=/)").unwrap();
                    let cut_file_path = re.find(file_path.to_str().unwrap())
                        .map_or("", |m| m.as_str());

                    let compressed_file_path = format!("{}/{}", cut_file_path, dragon_file);
                    match fs::write(compressed_file_path, compressed_data) {
                        Ok(()) => println!("File was successfully created"),
                        Err(e) => println!("There was an error {}", e)
                    }
                
                }
            }
            ui.label("TEST");
            if ui.button("Decompress File").clicked() {
                println!("Decompress File Button Clicked");
                let file = FileDialog::new()
                    .add_filter("Dragon Files", &["dragon"])
                    .set_directory("/")
                    .pick_file();
            }
        });
    }
}

impl eframe::App for Dragon {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to Dragon");
            self.get_file(ui);
        });
    }
}