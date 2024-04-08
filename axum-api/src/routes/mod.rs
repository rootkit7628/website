pub mod award;
pub mod experience;
pub mod formation;
pub mod project;
pub mod skill;
pub mod user;

fn logging(log: &str) {
    let timestamp = chrono::Local::now().format("%F %T").to_string();
    println!("INFO:     {} - \"{}\"", timestamp, log);
}
