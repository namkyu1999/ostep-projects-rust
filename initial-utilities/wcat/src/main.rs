use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("commands are required");
        return; 
    }
    for filename in args.iter().skip(1) {
        let mut file = match File::open(filename) {
            Err(why) => panic!("couldn't open {}: {}", filename, why),
            Ok(file) => file,
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(why) => panic!("couldn't read {}: {}", filename, why),
            Ok(_) => println!("{}", contents),
        }
    }
}