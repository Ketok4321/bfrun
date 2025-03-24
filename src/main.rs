use std::env;
use std::fs::File;
use std::io::{self, Read};

use bfrun::interpreter::*;
use bfrun::parser::*;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: bfrun <path_to_file>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    let mut file = File::open(file_path)?;

    let mut code = String::new();
    file.read_to_string(&mut code)?;

    let code = parse(&code);
    run(30000, &code);

    Ok(())
}
