extern crate json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
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