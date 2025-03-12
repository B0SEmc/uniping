mod parser;
mod stats;
mod tcp;

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
    pub n_threads: std::num::NonZero<usize>,
}

impl<'a> Settings<'a> {
    pub fn new(
        ip: &'a str,
        port: u16,
        interval: u64,
        number: u64,
        d_flag: bool,
        quiet: bool,
        n_threads: std::num::NonZero<usize>,
    ) -> Self {
        Settings {
            ip,
            port,
            interval,
            number,
            d_flag,
            quiet,
            n_threads,
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
        true
    }
}

impl Default for Settings<'_> {
    fn default() -> Self {
        Self::new(
            "",
            DEFAULT_PORT,
            DEFAULT_INTERVAL_MS,
            u64::MAX,
            false,
            false,
            std::thread::available_parallelism().unwrap(),
        )
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
    let mut settings: Settings = Settings::default();
    let mut results: Vec<u128> = Vec::new();
    parse(&mut settings, &args);
    my_tcping(&settings, &mut results);
    print_stats(&results, settings.ip);
}
