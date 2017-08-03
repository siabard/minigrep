use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Config {
    query: String,
    filename: String,
}
fn parse_config(args: &[String]) -> Config {

    let query = args[1].clone();
    let filename = args[2].clone();
    Config{query, filename}
}

fn main() {

    // Accept command line arguments
    let args:Vec<String> = env::args().collect();

    let config:Config = parse_config(&args);

    println!("Query : {}", config.query);
    println!("FileName: {}", config.filename);

    // Reading file
    let mut f = File::open(config.filename).expect("File cannot open");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something wrong with reading file");
    
    println!("With text:\n{}", contents);
    //println!("{:?}", args);
}
