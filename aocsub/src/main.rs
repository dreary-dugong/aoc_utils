use aocsub::Config;
use std::process;
fn main() {
    if let Err(e) = aocsub::run(Config::make()) {
        eprintln!("ERROR {}", e);
        process::exit(1);
    }
    process::exit(0);
}
