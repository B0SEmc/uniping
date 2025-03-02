use std::process::exit;

use crate::{Settings, print_help};

pub fn parse<'a>(settings: &mut Settings<'a>, args: &'a [String]) {
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-d" => settings.switch_to_d_flag(),
            "-h" | "--help" => {
                print_help();
                exit(0);
            }
            "-q" | "--quiet" => settings.quiet = true,
            "-p" | "--port" => {
                if i + 1 >= args.len() {
                    println!("Invalid port was given !");
                    exit(84);
                }
                settings.port = match args[i + 1].parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid port was given !");
                        exit(84);
                    }
                };
                i += 1;
            }
            "-i" | "--interval" => {
                if i + 1 >= args.len() {
                    println!("Invalid interval time was given !");
                    exit(84);
                }
                settings.set_interval(match args[i + 1].parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid interval time was given !");
                        exit(84);
                    }
                });
                i += 1;
            }
            "-n" | "--number" => {
                if i + 1 >= args.len() {
                    println!("Invalid number of requests was given !");
                    exit(84);
                }
                settings.number = match args[i + 1].parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("Invalid number of requests was given !");
                        exit(84);
                    }
                };
                i += 1;
            }
            _ => {
                if settings.ip.is_empty() {
                    settings.ip = &args[i];
                } else {
                    if let Some(char) = args[i].chars().next() {
                        if char == '-' {
                            println!("Unknown flag : {} !", args[i]);
                            exit(84);
                        }
                    };
                    println!("Invalid usage ! more than one target was given !");
                    exit(84);
                }
            }
        }
        i += 1;
    }
}
