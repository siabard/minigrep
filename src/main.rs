extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

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

    // if let Err(e) = minigrep::run(config) {
    //     println!("Application error: {}", e);

    //     process::exit(1);
    // }

    minigrep::run(config).unwrap_or_else(|e| {
        println!("Application error: {}", e);
        process::exit(1);
    });
}
