use std::{
    io::{Read, Write},
    net::TcpStream,
    thread::sleep,
    time::{Duration, Instant},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <host> <port>", args[0]);
        return;
    }
    loop {
        let mut stream = TcpStream::connect(format!("{}:{}", args[1], args[2]))
            .expect("Error connecting to address!");
        dbg!(stream.peer_addr().unwrap());
        let now = Instant::now();
        stream.write_all(&[1]).unwrap();
        let mut buffer: [u8; 1] = [0; 1];
        stream.read_exact(&mut buffer).unwrap();
        let elapsed = now.elapsed();
        println!(
            "Reply from {} ({}) on port {} took {:?}",
            args[1],
            stream.peer_addr().unwrap(),
            args[2],
            elapsed
        );
        sleep(Duration::from_secs(1));
    }
}
