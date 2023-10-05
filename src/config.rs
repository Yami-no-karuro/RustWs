#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn create_from_args(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Missing arguments: please provide a valid port.");
        }

        let port_str: &String = &args[1];
        let port: u16 = match port_str.parse() {
            Ok(parsed_port) if parsed_port >= 1 => parsed_port,
            _ => return Err("Invalid argument, port should be of type number."),
        };

        return Ok(Config { port });
    }
}
