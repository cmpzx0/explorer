use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let mut current_dir = env::current_dir().unwrap();

    loop {
        println!("\nCurrent directory: {}\n", current_dir.display());

        match fs::read_dir(&current_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        println!("[DIR]  {}", entry.file_name().to_string_lossy());
                    } else {
                        println!("       {}", entry.file_name().to_string_lossy());
                    }
                }
            }
            Err(e) => println!("Error reading directory: {}", e),
        }

        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let mut parts = input.split_whitespace();
        let command = parts.next();
        let argument = parts.next();

        match command {
            Some("cd") => {
                if let Some(path) = argument {
                    let new_path = current_dir.join(path);
                    if new_path.is_dir() {
                        current_dir = new_path.canonicalize().unwrap();
                    } else {
                        println!("Directory not found: {}", path);
                    }
                } else {
                    println!("Usage: cd <path>");
                }
            }
            Some("open") => {
                if let Some(file) = argument {
                    let file_path = current_dir.join(file);
                    if file_path.exists() {
                        Command::new("cmd")
                            .args(["/C", "start", "", &file_path.to_string_lossy()])
                            .spawn()
                            .expect("Failed to open file");
                    } else {
                        println!("File not found: {}", file);
                    }
                } else {
                    println!("Usage: open <file>");
                }
            }
            Some("pwd") => {
                println!("Current directory: {}", current_dir.display());
            }
            Some("find") => {
                if let Some(name) = argument {
                    println!("Searching for \"{}\"...\n", name);
                    find_recursively(&current_dir, name);
                } else {
                    println!("Usage: find <name>");
                }
            }
            Some("exit") => {
                println!("Exiting...");
                break;
            }
            Some(cmd) => {
                println!("Unknown command: {}", cmd);
            }
            None => {}
        }
    }
}

fn find_recursively(dir: &Path, name: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_lowercase();

            if file_name.contains(&name.to_lowercase()) {
                println!("{}", path.display());
            }

            if path.is_dir() {
                find_recursively(&path, name);
            }
        }
    }
}
