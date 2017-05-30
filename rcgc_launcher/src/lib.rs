

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
