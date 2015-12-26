extern crate rustc_serialize;
use rustc_serialize::json::Json;

use std::process;
use std::io::{self, Read};

fn main() {
    let mut stdin = io::stdin();
    let mut json_buffer = &mut String::new();

    match stdin.read_to_string(&mut json_buffer) {
        Ok(_) => {},
        Err(_) => {
            println!("Unable to read from stdin");
            process::exit(1);
        },
    }

    match Json::from_str(json_buffer) {
        Ok(json) => {
            println!("{}", json.pretty());
        },
        Err(e) => {
            println!("Unable to parse json. Err: {}", e);
            process::exit(1);
        }
    }
}
