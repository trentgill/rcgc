extern crate rcgc_launcher;
extern crate json;

use std::env;
use std::process::Command;
use rcgc_launcher::Config;
use rcgc_launcher::unwrap_or_stderr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = unwrap_or_stderr(
        Config::new(&args),
        "Problem parsing arguments"
    );

    let json_path = format!("{}config.json", config.path);
    let json = unwrap_or_stderr(
        rcgc_launcher::path_to_json(&json_path),
        "Problem parsing config.json"
    );
    
    let engines_path = String::from("engines.json");
    let engines = unwrap_or_stderr(
        rcgc_launcher::path_to_json(&engines_path),
        "Problem parsing engines.json"
    );

    let engine = json["engine"].as_str().unwrap_or("");
    let interpreter_cmd = engines[engine].as_str().unwrap_or("");
    
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
