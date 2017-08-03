use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}


fn main() {

    // Accept command line arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
                                                       println!("Problem parsing arguments: {}",
                                                                err);
                                                       process::exit(1);
                                                   });

    println!("Query : {}", config.query);
    println!("FileName: {}", config.filename);

    // if let Err(e) = run(config) {
    //     println!("Application error: {}", e);

    //     process::exit(1);
    // }

    run(config).unwrap_or_else(|e| {
        println!("Application error: {}", e);
        process::exit(1);
    });
}

fn run(config: Config) -> Result<(), Box<Error>>{

    // Reading file
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);
   
    Ok(())
}