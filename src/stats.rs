pub fn print_stats(results: &[u128], ip: &str) {
    if results.len() == 0 {
        return;
    }
    let max = *results.iter().max().unwrap() as f64;
    let min = *results.iter().min().unwrap() as f64;
    let len = results.len() as u128;
    let stats = format!(
        "Number of requests: {}\n\tmin: {:.3} ms\n\tavg: {:.3} ms\n\tmax: {:.3} ms\n\tmaxdev: {:.3} ms",
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
