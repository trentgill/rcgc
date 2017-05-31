extern crate rcgc_launcher;
extern crate json;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::process::Command;
use rcgc_launcher::Config;
use json::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stderr = std::io::stderr();
    let config = Config::new(&args).unwrap_or_else( |err| {
        writeln!(
            &mut stderr,
            "Problem parsing arguments: {}",
            err
            ).expect("Unable to write to stderr");
        process::exit(1);
    });

    //println!("Argument: {}", config.path);
    let message = format!("echo {}", config.path);
    let output = Command::new("sh")
            .arg("-c")
            .arg(message)
            .output()
            .expect("Unable to execute shell command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
    
    let json = rcgc_launcher::parse_json(&config.path).unwrap_or_else( |err| {
        writeln!(
            &mut stderr,
            "Problem parsing json: {}",
            err
            ).expect("Unable to write to stderr");
        process::exit(1);
    });
    println!("author: {}", json["author"]);
}
