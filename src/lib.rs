mod models;
mod modules;

use std::error::Error;
use std::fs;

use modules::tokenizer;

// TODO: Implement Error handling structure

#[derive(Debug)]
pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("NoFileNameError");
        }

        let filename_extension: Vec<&str> = args[1].split('.').collect();

        if filename_extension.len() < 2 {
            return Err("InvalidExtensionError");
        }

        let filename = filename_extension[0].clone();
        let extension = filename_extension[1].clone();
        if extension == "fer" {
            Ok(Config {
                filename: String::from(filename) + "." + extension,
            })
        } else {
            Err("InvalidExtensionError")
        }
    }

    // NOTE: program is being extracted in String format
    pub fn run(file: String) -> Result<(), Box<dyn Error>> {
        let program = fs::read_to_string(file)?;
        let tokens = tokenizer::lexer(program)?;
        for token in tokens {
            println!("{:?}", token.token_type);
            println!("{:?}", token.value);
            println!("")
        }
        Ok(())
    }
}
