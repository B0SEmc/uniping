use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

mod parser;
use parser::{Args, parse};

const DEFAULT_PORT: u16 = 80;
const DEFAULT_INTERVAL_MS: u64 = 1000;

fn print_stats(results: &[u128], ip: &str) {
    let max = *results.iter().max().unwrap() as f64;
    let min = *results.iter().min().unwrap() as f64;
    let len = results.len() as u128;
    let stats = format!(
        "Packets: {} min: {:.3} avg: {:.3} max: {:.3} maxdev: {:.3}",
        len,
        min / 1000.0,
        (results.iter().sum::<u128>() / len) as f64 / 1000.0,
        max / 1000.0,
        (max - min) / 1000.0
    );
    println!();
    (0..stats.len() / 2 - 11 - ip.len() / 2).for_each(|_| print!("-"));
    print!(" {} uniping statistics ", ip);
    (0..stats.len() / 2 - 11 - ip.len() / 2).for_each(|_| print!("-"));
    println!("\n{}", stats);
}

fn my_tcping(ip: &str, interval: u64, port: u16) {
    let mut results: Vec<u128> = Vec::new();
    let (tx, rx) = channel::<bool>();
    ctrlc::set_handler(move || tx.send(true).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    let mut exit = false;
    println!(
        "Pinging {} with {} ms of interval on port {}",
        ip, interval, port
    );
    while !exit {
        let mut stream =
            TcpStream::connect(format!("{}:{}", ip, port)).expect("Error connecting to address!");
        let now = Instant::now();
        stream.write_all(&[1]).unwrap();
        let mut buffer: [u8; 1] = [0; 1];
        stream.read_exact(&mut buffer).unwrap();
        let elapsed = now.elapsed();
        results.push(elapsed.as_micros());
        println!(
            "Reply from {} ({}) on port {} took {:.3} ms",
            ip,
            stream.peer_addr().unwrap(),
            port,
            ((elapsed.as_micros() as u32) as f64) / 1000.0
        );
        if rx.recv_timeout(Duration::from_millis(interval)).is_ok() {
            exit = true;
        }
    }
    print_stats(&results, ip);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut settings = Args {
        ip: &args[1],
        port: DEFAULT_PORT,
        interval: DEFAULT_INTERVAL_MS,
    };
    parse(&mut settings, &args);
    my_tcping(settings.ip, settings.interval, settings.port);
}
