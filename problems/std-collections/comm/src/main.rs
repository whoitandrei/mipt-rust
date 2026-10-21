#![forbid(unsafe_code)]

use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err("usage: comm <file1> <file2>".into());
    }

    let mut first = HashSet::<String>::new();
    for line in BufReader::new(File::open(&args[1])?).lines() {
        first.insert(line?);
    }

    for line in BufReader::new(File::open(&args[2])?).lines() {
        if let Some(common) = first.take(&line?) {
            println!("{}", common);
        }
    }
    Ok(())
}
