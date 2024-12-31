use std::env;
use std::fs;
use std::process::Command;
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
                let files = FileDialog::new()
                    .add_filter("All Files", &["*.*"])
                    .set_directory("/")
                    .pick_file();
            }
            ui.label("TEST");
            if ui.button("Decompress File").clicked() {
                println!("Decompress File Button Clicked");
                let files = FileDialog::new()
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