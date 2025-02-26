use std::{
    io::{Read, Write},
    net::TcpStream,
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let default_interval_time = 10;
    let args: Vec<String> = std::env::args().collect();
    let port: u32 = match args.len() {
        2 => 80,
        3 => match args[2].parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid argument was given !");
                exit(84)
            }
        },
        _ => {
            println!("Usage: {} <host> <port>", args[0]);
            exit(84)
        }
    };
    loop {
        let mut stream = TcpStream::connect(format!("{}:{}", args[1], port))
            .expect("Error connecting to address!");
        let now = Instant::now();
        stream.write_all(&[1]).unwrap();
        let mut buffer: [u8; 1] = [0; 1];
        stream.read_exact(&mut buffer).unwrap();
        let elapsed = now.elapsed();
        println!(
            "Reply from {} ({}) on port {} took {:?}",
            args[1],
            stream.peer_addr().unwrap(),
            port,
            elapsed
        );
        sleep(Duration::from_millis(default_interval_time));
    }
}
