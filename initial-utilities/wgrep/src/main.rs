use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("wgrep: searchterm [file ...]");
        return;
    }
    let term = &args[1];
    for filename in args.iter().skip(2){
        let mut file = match File::open(filename){
            Err(why) => panic!("couldn't open {}: {}", filename, why),
            Ok(file) => file,
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(why) => panic!("couldn't open {}: {}",filename, why),
            Ok(_) => {
                for content in contents.lines() {
                    if content.contains(term) {
                        println!("{}", content);
                    }
                }
            },
        };
    }
}
