extern crate json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use json::JsonValue;
use std::process;

mod parse_json;

pub struct ShellArgs {
    pub path: String,
}

impl ShellArgs {
    pub fn new(args: &[String]) -> Result<ShellArgs, &'static str> {
        if args.len() != 2 {
            return Err("wrong number of arguments");
        }
        let path = 
            if args[1].chars().last() == Some('/') {args[1].clone()}
            else {format!("{}/", &args[1])};
        Ok(ShellArgs {
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

/// Unwraps a Result or exits & prints Err as formatted string to stderr
///
/// # Examples
/// ```
/// let good_result: Result<i32, str> = Ok(10);
/// let bad_result: Result<i32, str> = Err(":(");
///
/// let foo = unwrap_or_stderr(
///     good_result,
///     "Received a bad result"
/// );
/// assert!(foo == 10);
///
/// let bar = unwrap_or_stderr(
///     bad_result,
///     "Received a bad result"
/// );
/// // Program will exit due to failure to unwrap
/// // and print to stderr "Received a bad result: :("
/// ```
pub fn unwrap_or_stderr<T, E>(burrito: Result<T, E>, msg: &str) -> T
        where E: std::fmt::Display {
    burrito.unwrap_or_else( |err| {
        writeln!(
            std::io::stderr(),
            "{}: {}",
            msg,
            err
            ).expect("Unable to write to stderr");
        process::exit(1);
    })
}

/// Checks a config.json for required peripherals, exits with an error unless they're connected.
///
/// This function can take any &JsonValue as an input, but should only be called on config.json.
/// Poorly formatted config.json files may cause this function to panic. This will happen if
/// the key paired with a "required" value in a key-value pair in a JSON object in config.json starts with a non-ascii character.
///
/// This function spawns two child processes, one at a time. The first one runs `ls
/// /dev/input/by-id`, storing the result as `ls_out`. The second runs `grep -c -m 1 [Kk]ey`,
/// with `ls_out` piped to stdin and where `[Kk]ey` is replaced by the name of the required
/// peripheral (such as `[Kk]eyboard`).
/// # Example
/// ```
/// let path_to_config = format!("{}config.json", shell_args.path);
/// let config = unwrap_or_stderr(
///     rcgc_launcher::path_to_json(&path_to_config),
///     "Problem parsing config.json"
/// );
/// rcgc_launcher::check_peripherals(&config);
///
/// ```
pub fn check_peripherals(config: &JsonValue) {
    for (key, value) in config.entries() {
        if value == "required" {
            let first_char = key.chars().nth(0).unwrap();
            let len = key.len();
            let short_key = String::from(&key[1..len]);
            let ls_arg = "/dev/input/by-id"; 
            let grep_arg = format!(
                "[{}{}]{}",
                first_char.to_uppercase(),
                first_char.to_lowercase(),
                short_key
            );
            let ls_command = Command::new("ls")
                .arg(&ls_arg)
                .output()
                .expect("Peripheral check failed to execute ls command");
            let ls_out = &ls_command.stdout;
            let mut grep_command = Command::new("grep")
                .args(&["-c", "-m 1", &grep_arg])
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Peripheral check failed to execute grep command");
            {
                let grep_in = grep_command.stdin
                    .as_mut()
                    .expect("Peripheral check failed to get grep stdin");
                grep_in.write_all(ls_out)
                    .expect("Peripheral check failed to write to grep stdin");
            }
            let grep_out = grep_command
                .wait_with_output()
                .expect("Peripheral check failed to wait on grep process");
            let peripheral_exists = String::from_utf8_lossy(
                &grep_out.stdout);
            if peripheral_exists.trim() == "0" {
                writeln!(
                    std::io::stderr(),
                    "Required peripheral: '{}' is missing! Try plugging one in.",
                    key
                ).expect("Unable to write to stderr");
                process::exit(1);
            }
        }
    }
}
