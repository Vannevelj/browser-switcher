use std::env;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <string>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    println!("Received argument: {}", input);

    match get_chrome_path() {
        Some(chrome_path) => {
            let status = Command::new(chrome_path)
                .arg(input)
                .status();

            match status {
                Ok(status) if status.success() => println!("Chrome launched successfully!"),
                Ok(status) => eprintln!("Chrome exited with status: {}", status),
                Err(e) => eprintln!("Failed to launch Chrome: {}", e),
            }
        }
        None => eprintln!("Chrome not found on this system."),
    }
}

fn get_chrome_path() -> Option<&'static str> {
    let paths = [
        "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
        "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
    ];

    for path in paths {
        if Path::new(path).exists() {
            return Some(path);
        }
    }

    None
}