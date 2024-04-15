use std::{
    env::{self},
    process,
};

use minigrepiter::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err: &str| {
        eprintln!("Problem parsing arguments :{err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error:{}", e);
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
