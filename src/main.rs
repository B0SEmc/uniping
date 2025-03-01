mod parser;
mod stats;
mod tcp;

use parser::{Args, parse};
use stats::print_stats;
use tcp::my_tcping;

const DEFAULT_PORT: u16 = 80;
const DEFAULT_INTERVAL_MS: u64 = 1000;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut settings = Args {
        ip: &args[1],
        port: DEFAULT_PORT,
        interval: DEFAULT_INTERVAL_MS,
    };
    let mut results: Vec<u128> = Vec::new();
    parse(&mut settings, &args);
    my_tcping(settings.ip, settings.interval, settings.port, &mut results);
    print_stats(&results, settings.ip);
}
