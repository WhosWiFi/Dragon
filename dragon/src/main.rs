use std::env;
use std::fs;
use std::process::Command;
use std::str;
use regex::Regex;
use eframe::egui;
use rfd::FileDialog;

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

fn compress_text(data:Vec<u8>) {
    println!("Dragon Processed {} bytes", data.len());

    let dictionary_common_words: Vec<(&str, &[u8])> = vec![
        ("the",  &[0x01]),
        ("and",  &[0x02]),
        ("all",  &[0x03]),
        ("ight", &[0x04]),
        ("tion", &[0x05]),
        ("ment", &[0x06]),
        ("ness", &[0x07]),
        ("ship", &[0x08]),
        ("able", &[0x09]),
        ("ance", &[0x0A]),
        ("ence", &[0x0B]),
        ("ture", &[0x0C]),
        ("ward", &[0x0D]),
        ("ing",  &[0x0E]),
        ("ion",  &[0x0F]),
        ("ate",  &[0x10]),
        ("ive",  &[0x11]),
        ("ize",  &[0x12]),
        ("ful",  &[0x13]),
        ("ous",  &[0x14]),
        ("est",  &[0x15])
    ];

    let mut iterator = 0;
    while iterator < data.len() {
        if data[i] == 't' {
            
        }
    }

        // Define patterns and their replacements
    let patterns: Vec<String> = vec![
        "the".to_string(),
        "and".to_string(),
        "all".to_string(),
        "ight".to_string(),
        "tion".to_string(),
        // ... other patterns
    ];

    // Define replacements (single bytes)
    let replacements: Vec<u8> = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, // ... etc
    ];

    // Build Aho-Corasick automaton
    let ac = AhoCorasick::new(&patterns);
    let mut compressed = Vec::with_capacity(data.len());
    let mut last_match_end = 0;

    // Find all matches
    for mat in ac.find_iter(&data) {
        // Copy bytes before the match
        compressed.extend_from_slice(&data[last_match_end..mat.start()]);
        // Add the replacement byte
        compressed.push(replacements[mat.pattern()]);
        last_match_end = mat.end();
    }

    // Copy remaining bytes
    compressed.extend_from_slice(&data[last_match_end..]);
    compressed


    for (pattern, identifier) in dictionary_common_words.iter() {
        file_contents_string = file_contents_string.replace(pattern, identifier);
    }

    let compressed_file = file_contents_string.as_bytes().to_vec();
    println!("Dragon compressed the file from {} bytes to {} bytes", data.len(), compressed_file.len());
    
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
                    match fs::read(file_path) {
                        Ok(data) => compress_text(data),
                        Err(e) => println!("Error reading file: {}", e),
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