use std::process;
fn main() {
    if let Err(e) = run(Config::make()) {
        eprintln!("{}", e);
        process::exit(1);
    } else {
        process::exit(0);
    }
}
