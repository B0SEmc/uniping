use std::{
    io::{Read, Write},
    net::TcpStream,
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

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
    let port: u16 = match args.len() {
        2 | 3 => 80,
        4 => match args[3].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid argument was given !");
                exit(84)
            }
        },
        _ => {
            println!("Usage: {} <host> [<interval> <port>]", args[0]);
            exit(84)
        }
    };
    let interval_time: u64 = match args.len() {
        2 => 1000,
        3 | 4 => match args[2].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid argument was given !");
                exit(84)
            }
        },
        _ => {
            println!("Usage: {} <host> [<interval> <port>]", args[0]);
            exit(84)
        }
    };
    my_tcping(&args[1], interval_time, port);
}
