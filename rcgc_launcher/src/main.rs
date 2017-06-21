extern crate rcgc_launcher;
extern crate json;

use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::process::Command;
use rcgc_launcher::ShellArgs;
use rcgc_launcher::unwrap_or_stderr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let shell_args = unwrap_or_stderr(
        ShellArgs::new(&args),
        "Problem parsing arguments"
    );

    let path_to_config = format!("{}config.json", shell_args.path);
    let config = unwrap_or_stderr(
        rcgc_launcher::path_to_json(&path_to_config),
        "Problem parsing config.json"
    );
    
    let path_to_engines = String::from("engines.json");
    let available_engines = unwrap_or_stderr(
        rcgc_launcher::path_to_json(&path_to_engines),
        "Problem parsing engines.json"
    );

    let selected_engine = config["engine"]
        .as_str().unwrap_or("");
    let engine_cmd = available_engines[selected_engine]
        .as_str().unwrap_or("");
    
    rcgc_launcher::check_devices(&config);
    
    //eg: how to run a formatted shell command in a sub-terminal
    //println!("Argument: {}", shell_args.path);
    match selected_engine {
        "Web" => {
            // echo 'chromium-browser --app=http://example.com' >
            // ~/.xinitrc && startx && rm ~/.xinitrc
            let url = config["path"].as_str().unwrap_or("");
            let web_cmd = format!("chromium-browser --app={}", url);
            let web_cmd_bytes = web_cmd.into_bytes();
            let mut xinitrc = unwrap_or_stderr(
                File::create("~/.xinitrc"),
                "Unable to create ~/.xinitrc"
            );
            unwrap_or_stderr(
                xinitrc.write_all(&web_cmd_bytes),
                "Unable to write to ~/.xinitrc"
            );
            xinitrc.sync_all().unwrap();

            Command::new("startx")
                .output()
                .expect("Unable to startx");
            Command::new("rm")
                .arg("~/.xinitrc")
                .output()
                .expect("Unable to rm ~/.xinitrc");
        }
        _ => {
            let launch_cmd = format!(
                "'{} {}'",
                engine_cmd,
                shell_args.path
            );
            let output = Command::new("sh")
                .arg("-c")
                .arg(launch_cmd)
                .output()
                .expect("Unable to execute shell command");
            println!("output: {}", String::from_utf8_lossy(&output.stdout));
        }
    }
}
