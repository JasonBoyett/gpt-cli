use dotenv::dotenv;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::{env, io};

pub fn get_api_key() -> Result<String, Box<dyn Error>> {
    dotenv().ok();

    match env::var("API_KEY") {
        Ok(val) => {
            let pattern = match Regex::new(r#"API_KEY="(\s*)""#) {
                Ok(expression) => expression,
                Err(e) => panic!("Error: {}", e),
            };

            if !pattern.is_match(&val) {
                eprintln!("Please add your API_KEY to the .env file");
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "API_KEY not found in .env file",
                )));
            }

            return Ok(val);
        }
        Err(_) => match ensure_env_file() {
            Ok(_) => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    "API_KEY not found in .env file",
                )))
            }
            Err(e) => return Err(e),
        },
    };
}

fn ensure_env_file() -> Result<(), Box<dyn Error>> {
    let path = Path::new(".env");

    let content = match fs::read_to_string(path) {
        Ok(val) => val,
        Err(_) => String::from(""),
    };

    if !content.contains("API_KEY=") {
        match std::fs::write(path, "API_KEY=\"\"") {
            Ok(_) => {
                println!("Please add your API_KEY to the new .env file");
                return Ok(());
            }
            Err(e) => return Err(Box::new(e)),
        };
    }

    return Ok(());
}
