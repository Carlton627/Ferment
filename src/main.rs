use std::env;
use std::process;

use ferment::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        if err == "NoFileNameError" {
            println!("Docs");
        } else if err == "InvalidExtensionError" {
            println!("Can't process files with invalid extension");
        } process::exit(0);
    });

    if let Err(e) = Config::run(config.filename) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
