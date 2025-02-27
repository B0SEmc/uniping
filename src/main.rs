use std::{
    io::{Read, Write},
    net::TcpStream,
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

const DEFAULT_PORT: u16 = 80;
const DEFAULT_INTERVAL_MS: u64 = 1000;

fn my_tcping(ip: &str, interval: u64, port: u16) {
    let mut results: Vec<u128> = Vec::new();
    loop {
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
        sleep(Duration::from_millis(interval));
        if false {
            break;
        }
    }
    let max = *results.iter().max().unwrap() as f64;
    let min = *results.iter().min().unwrap() as f64;
    let len = results.len() as u128;
    println!(
        "Packets: {} min/avg/max/maxdev {:.3}/{:.3}/{:.3}/{:.3}",
        len,
        min / 1000.0,
        (results.iter().sum::<u128>() / len) as f64 / 1000.0,
        max / 1000.0,
        (max - min) / 1000.0
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ip: &str = &args[1];
    let mut port: u16 = DEFAULT_PORT;
    let mut interval_ms: u64 = DEFAULT_INTERVAL_MS;
    match args.len() {
        2 => (),
        3 => {
            interval_ms = match args[2].parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid argument was given !");
                    exit(84)
                }
            }
        }
        4 => {
            port = match args[3].parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Invalid argument was given !");
                    exit(84)
                }
            };
            interval_ms = match args[2].parse() {
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
    my_tcping(ip, interval_ms, port);
}
