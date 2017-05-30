extern crate json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use json::JsonValue;

mod path_to_json;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("wrong number of arguments");
        }
        let path = args[1].clone();
        Ok(Config {
            path: path,
        })
    }
}

pub fn parse_json(path: &String) -> Result<JsonValue, Box<Error>> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json = path_to_json::path_to_json(&contents)?;
    Ok(json)
}
