mod parser;
mod stats;
mod tcp;

use std::u64;

use parser::parse;
use stats::print_stats;
use tcp::my_tcping;

const DEFAULT_PORT: u16 = 80;
const DEFAULT_INTERVAL_MS: u64 = 1000;

pub struct Settings<'a> {
    pub ip: &'a str,
    pub port: u16,
    pub interval: u64,
    pub number: u64,
    pub d_flag: bool,
    pub quiet: bool,
}

impl<'a> Settings<'a> {
    pub fn create() -> Self {
        Settings {
            ip: "",
            port: DEFAULT_PORT,
            interval: DEFAULT_INTERVAL_MS,
            number: u64::MAX,
            d_flag: false,
            quiet: false,
        }
    }
    pub fn switch_to_d_flag(&mut self) {
        self.d_flag = true;
        self.interval = 0;
    }
    pub fn set_interval(&mut self, interval: u64) -> bool {
        if self.d_flag {
            return false;
        }
        self.interval = interval;
        return true;
    }
}

pub fn print_help() {
    println!("USAGE:\n\t./uniping TARGET [[flags] [args]]");
    println!("\nDESCRIPTION:");
    println!("\t-h | --help\tPrint this menu.");
    println!("\t-q | --quiet\tDoes not print the result of each requests.");
    println!("\t-p | --port\tChoose a specific port to ping.");
    println!("\t-i | --interval\tChoose a specific interval time between each requests.");
    println!("\t-n | --number\tNumber of requests to send.");
    println!("\t-d\t\tSecret opton.");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut settings: Settings = Settings::create();
    let mut results: Vec<u128> = Vec::new();
    parse(&mut settings, &args);
    my_tcping(&settings, &mut results);
    print_stats(&results, settings.ip);
}
