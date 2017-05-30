extern crate rcgc_launcher;
extern crate json;

use std::env;
use std::process;
use std::process::Command;
use rcgc_launcher::Config;
use json::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else( |err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("Argument: {}", config.path);
    let message = format!("echo {}", config.path);
    let output = Command::new("sh")
            .arg("-c")
            .arg(message)
            .output()
            .expect("failed to execute process");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
