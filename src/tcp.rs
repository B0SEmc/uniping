use std::{
    io::{Read, Write},
    net::TcpStream,
    sync::mpsc::channel,
    time::{Duration, Instant},
};

use crate::Settings;

pub fn my_tcping(settings: &Settings, results: &mut Vec<u128>) {
    let (tx, rx) = channel::<bool>();
    let mut i = 0;
    ctrlc::set_handler(move || tx.send(true).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");
    let mut exit = false;
    println!(
        "Pinging {} on port {} with {} ms of interval",
        settings.ip, settings.port, settings.interval
    );
    while !exit && i < settings.number {
        if rx
            .recv_timeout(Duration::from_millis(settings.interval))
            .is_ok()
        {
            exit = true;
        }
        let mut stream = TcpStream::connect(format!("{}:{}", settings.ip, settings.port))
            .expect("Error connecting to address!");
        let mut buffer: [u8; 1] = [0; 1];
        let now = Instant::now();
        stream.write_all(&[1]).unwrap();
        stream.read_exact(&mut buffer).unwrap();
        let elapsed = now.elapsed();
        results.push(elapsed.as_micros());
        if !settings.quiet {
            println!(
                "Reply from {} ({}) on port {} took {:.3} ms",
                settings.ip,
                stream.peer_addr().unwrap(),
                settings.port,
                (elapsed.as_micros() as f64) / 1000.0
            );
        }
        i += 1;
    }
}
