use std::env;
use std::fs::File;
use std::io::prelude::*;


fn main() {

    // Accept command line arguments
    let args:Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Query : {}", query);
    println!("FileName: {}", filename);

    // Reading file
    let mut f = File::open(filename).expect("File cannot open");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something wrong with reading file");
    
    println!("With text:\n{}", contents);
    //println!("{:?}", args);
}
