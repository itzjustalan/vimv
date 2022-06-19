use std::env;
use std::process;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if let Err(e) = vimv::run(&mut args) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

