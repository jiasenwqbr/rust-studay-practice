use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err: &str| {
        //println!("Problem parsing arguments: {err}");
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error:{}", e);
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
