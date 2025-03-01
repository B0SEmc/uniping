use std::process::exit;

pub struct Args<'a> {
    pub ip: &'a str,
    pub port: u16,
    pub interval: u64,
}

pub fn parse(settings: &mut Args, args: &[String]) {
    match args.len() {
        2 => (),
        3 => {
            settings.interval = match args[2].parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid argument was given !");
                    exit(84)
                }
            }
        }
        4 => {
            settings.port = match args[3].parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid argument was given !");
                    exit(84)
                }
            };
            settings.interval = match args[2].parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid argument was given !");
                    exit(84)
                }
            }
        }
        _ => {
            println!("Usage: {} <host> [<interval> <port>]", args[0]);
            exit(84)
        }
    }
}
