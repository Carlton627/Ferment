use std::env;
use std::process;

use ferment::extension_checker;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = extension_checker(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("{}", file);

    if let Err(e) = ferment::run(file) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
