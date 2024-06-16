mod connector;
use std::io::{self, ErrorKind};

fn main() {
    let api_key = match connector::environment::get_api_key() {
        Ok(val) => val,
        Err(e) => {
            let error = match e.downcast_ref::<io::Error>() {
                Some(kind) => kind,
                None => {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            };

            if error.kind() == ErrorKind::Other {
                eprintln!("Please re-run the program once api key is entered");
                std::process::exit(1);
            }
            std::process::exit(1);
        }
    };

    println!("API Key: {}", api_key);
}
