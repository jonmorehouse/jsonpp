extern crate rustc_serialize;
use rustc_serialize::json::Json;

extern crate argparse;
use argparse::{ArgumentParser, StoreTrue};

use std::process;
use std::io::{self, Read};


fn main() {
    let mut lined = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("JSON pretty printer");
        parser.refer(&mut lined)
            .add_option(&["-l", "--lined"], StoreTrue,
            "Parse each line as its own JSON object");
        parser.parse_args_or_exit();
    }

    
    let mut stdin = io::stdin();
    let mut json_buffer = &mut String::new();

    match stdin.read_to_string(&mut json_buffer) {
        Ok(_) => {},
        Err(_) => {
            println!("Unable to read from stdin");
            process::exit(1);
        },
    }

    {
        let jsons: Vec<&str>;
        if lined {
            jsons = json_buffer.split_terminator('\n').collect();
        } else {
            jsons = vec![json_buffer];
        }
        
        for json in jsons {
            match Json::from_str(json) {
                Ok(json) => {
                    println!("{}", json.pretty());
                },
                Err(e) => {
                    println!("Unable to parse json. Err: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
