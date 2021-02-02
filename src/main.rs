use std::env;
use std::process;

use ferment::extension_checker;

fn main() {
    let args: Vec<String> = env::args().collect();
    // TODO: add no file args error

    let file = extension_checker(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(e) = ferment::run(file) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
