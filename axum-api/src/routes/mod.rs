pub mod project;


fn logging(log: &str) {
    let timestamp = chrono::Local::now().format("%F %T").to_string();
    println!("INFO:     {} - \"{}\"", timestamp, log);
}
