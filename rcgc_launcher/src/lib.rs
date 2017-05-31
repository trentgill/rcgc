extern crate json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use json::JsonValue;

mod parse_json;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("wrong number of arguments");
        }
        let path = 
            if args[1].chars().last() == Some('/') {args[1].clone()}
            else {format!("{}/", &args[1])};
        Ok(Config {
            path: path,
        })
    }
}

pub fn path_to_json(path: &String) -> Result<JsonValue, Box<Error>> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let json = parse_json::parse_json(&contents)?;
    Ok(json)
}
