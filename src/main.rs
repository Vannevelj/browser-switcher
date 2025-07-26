use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <string>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    println!("Received argument: {}", input);
}
