extern crate rcgc_launcher;
extern crate json;

use std::env;
//use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::process::Command;
use rcgc_launcher::Config;
//use json::JsonValue;

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


    let json_path = format!("{}config.json", config.path);
    let json = rcgc_launcher::path_to_json(&json_path)
        .unwrap_or_else( |err| {
            writeln!(
                &mut stderr,
                "Problem parsing json: {}",
                err
            ).expect("Unable to write to stderr");
            process::exit(1);
        });
    
    let engines_path = String::from("engines.json");
    let engines = rcgc_launcher::path_to_json(&engines_path)
        .unwrap_or_else( |err| {
            writeln!(
                &mut stderr,
                "Problem parsing json: {}",
                err
            ).expect("Unable to write to stderr");
            process::exit(1);
        });
    let engine = json["engine"].as_str().unwrap_or("");
    println!("engine string: {}", &engine);
    let interpreter_cmd = engines[engine].as_str().unwrap_or("");
    println!("engines[{}]: {}", &engine, &interpreter_cmd);
    println!("engines[\"Love2D\"]: {}", engines["Love2D"].dump());
 
    println!(
        "what the shell sees: {} {}{}",
        interpreter_cmd,
        config.path,
        json["path"]
    );
    
    //eg: how to run a formatted shell command in a sub-terminal
    //println!("Argument: {}", config.path);
    let message = format!(
        "{} {}",
        interpreter_cmd,
        config.path
    );
    let output = Command::new("sh")
            .arg("-c")
            .arg(message)
            .output()
            .expect("Unable to execute shell command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
