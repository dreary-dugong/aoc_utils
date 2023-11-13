use aocfetch::Config;
use std::process;
fn main() {
    if let Err(err) = aocfetch::run(Config::make()) {
        eprintln!("{}", err);
        process::exit(1);
    }
    process::exit(0);
}
