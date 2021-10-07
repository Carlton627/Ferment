mod models;
mod modules;

use std::error::Error;
use std::fs;

use modules::tokenizer;

/////////////////////////////// CONSTANTS ///////////////////////////////////
/// TODO: give constants a global scope across the crate

const NFNE: &str = "NFNE";
const INEXTE: &str = "INEXTE";

/////////////////////////////////////////////////////////////////////////////

// TODO: ErrorMessages needs to be kept separately in a file to be used across the crate

#[derive(Debug)]
pub struct ErrorMessages {
    pub error_type: String,
}

impl ErrorMessages {
    pub fn throw_error(&self) -> &'static str {
        if self.error_type == String::from(NFNE) {
            return self.no_file_name_error();
        } else if self.error_type == String::from(INEXTE) {
            return self.invalid_extension_error();
        } else {
            "a"
        }
    }

    fn no_file_name_error(&self) -> &'static str {
        "NoFileNameError"
    }

    fn invalid_extension_error(&self) -> &'static str {
        "InvalidExtensionError"
    }
}

#[derive(Debug)]
pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            let get_error_message = ErrorMessages {
                error_type: String::from(NFNE),
            };
            return Err(get_error_message.throw_error());
        }

        let filename_extension: Vec<&str> = args[1].split('.').collect();

        if filename_extension.len() < 2 {
            let get_error_message = ErrorMessages {
                error_type: String::from(INEXTE),
            };
            return Err(get_error_message.throw_error());
        }

        let filename = filename_extension[0].clone();
        let extension = filename_extension[1].clone();
        if extension == "fer" {
            Ok(Config {
                filename: String::from(filename) + "." + extension,
            })
        } else {
            let get_error_message = ErrorMessages {
                error_type: String::from(INEXTE),
            };
            Err(get_error_message.throw_error())
        }
    }

    // NOTE: program is being extracted in String format
    pub fn run(file: String) -> Result<(), Box<dyn Error>> {
        let program = fs::read_to_string(file)?;
        let tokens = tokenizer::lexer(program)?;
        for token in tokens {
            println!("{:?}", token.token_type);
            println!("{:?}", token.value);
            println!("");
        }
        Ok(())
    }
}
