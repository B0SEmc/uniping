use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

pub fn my_tcping(ip: &str, interval: u64, port: u16, results: &mut Vec<u128>) {
    let (tx, rx) = channel::<bool>();
    ctrlc::set_handler(move || tx.send(true).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    let mut exit = false;
    println!(
        "Pinging {} on port {} with {} ms of interval",
        ip, port, interval
    );
    while !exit {
        let mut stream =
            TcpStream::connect(format!("{}:{}", ip, port)).expect("Error connecting to address!");
        let mut buffer: [u8; 1] = [0; 1];
        let now = Instant::now();
        stream.write_all(&[1]).unwrap();
        stream.read_exact(&mut buffer).unwrap();
        let elapsed = now.elapsed();
        results.push(elapsed.as_micros());
        println!(
            "Reply from {} ({}) on port {} took {:.3} ms",
            ip,
            stream.peer_addr().unwrap(),
            port,
            (elapsed.as_micros() as f64) / 1000.0
        );
        if rx.recv_timeout(Duration::from_millis(interval)).is_ok() {
            exit = true;
        }
    }
}
