use std::process;

fn main() {
    if let Err(msg) = minigrep::run() {
        eprintln!("Problem parsing args: {msg}");
        process::exit(1);
    };
}
