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
    loop {
        let mut stream =
            TcpStream::connect(format!("{}:{}", ip, port)).expect("Error connecting to address!");
        let now = Instant::now();
        stream.write_all(&[1]).unwrap();
        let mut buffer: [u8; 1] = [0; 1];
        stream.read_exact(&mut buffer).unwrap();
        let elapsed = now.elapsed();
        println!(
            "Reply from {} ({}) on port {} took {:?}",
            ip,
            stream.peer_addr().unwrap(),
            port,
            elapsed
        );
        sleep(Duration::from_millis(interval));
    }
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
