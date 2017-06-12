extern crate json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::ffi::OsStr;
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

/// Checks a config.json for required devices, exits with an error unless they're connected.
///
/// This function can take any &JsonValue as an input, but should only be called on config.json.
/// Poorly formatted config.json files may cause this function to panic. This will happen if
/// the key paired with a "required" value in a key-value pair in a JSON object in config.json starts with a non-ascii character.
///
/// This function spawns two child processes, one at a time. The first one runs `ls
/// /dev/input/by-id`, storing the result as `ls_out`. The second runs `grep -c -m 1 [Kk]ey`,
/// with `ls_out` piped to stdin and where `[Kk]ey` is replaced by the name of the required
/// device (such as `[Kk]eyboard`).
/// # Example
/// ```
/// let path_to_config = format!("{}config.json", shell_args.path);
/// let config = unwrap_or_stderr(
///     rcgc_launcher::path_to_json(&path_to_config),
///     "Problem parsing config.json"
/// );
/// rcgc_launcher::check_devices(&config);
///
/// ```
pub fn check_devices(config: &JsonValue) {
    for (key, value) in config.entries() {
        if value == "required" {
            let first_cmd_out;
            let grep_pattern;
            match key { 
                "network" => {
                    // ping -q -c 1 github.com | grep -c '1 received'
                    let ping_command = Command::new("ping")
                        .args(&["-q", "-c 1", "github.com"])
                        .output()
                        .expect("Device check failed to execute ping command");
                    first_cmd_out = ping_command.stdout.clone();
                    grep_pattern = String::from("1 received");
                }
                _ => {
                    // ls /dev/input/by-id | grep -c -m 1 '[Kk]ey'
                    let first_char = key.chars().nth(0).unwrap();
                    let len = key.len();
                    let short_key = String::from(&key[1..len]);
                    let ls_arg = "/dev/input/by-id"; 
                    grep_pattern = format!(
                        "[{}{}]{}",
                        first_char.to_uppercase(),
                        first_char.to_lowercase(),
                        short_key
                    );
                    let ls_command = Command::new("ls")
                        .arg(&ls_arg)
                        .output()
                        .expect("Device check failed to execute ls command");
                    first_cmd_out = ls_command.stdout.clone();
                }
            }
            let grep_out = pipe_to_grep(&first_cmd_out, &["-c", "-m 1"], &grep_pattern);
            let device_exists = String::from_utf8_lossy(
                &grep_out.stdout);
            if device_exists.trim() == "0" {
                writeln!(
                    std::io::stderr(),
                    "Required device: '{}' is missing! Please connect a {}.",
                    key, key
                ).expect("Unable to write to stderr");
                process::exit(1);
            }
        }
    }
}

fn pipe_to_grep<I, S>(grep_in: &Vec<u8>, flags: I, pattern: &String) -> process::Output
    where I: IntoIterator<Item = S>,
          S: AsRef<OsStr>
{
    let mut grep_command = Command::new("grep")
        .args(flags)
        .arg(pattern)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Device check failed to spawn grep command");
    {
        let stdin = grep_command.stdin
            .as_mut()
            .expect("Device check failed to get grep stdin");
        stdin.write_all(grep_in)
            .expect("Device check failed to write to grep stdin");
    }
    let grep_out = grep_command
        .wait_with_output()
        .expect("Device check failed to wait on grep process");
    grep_out
}
