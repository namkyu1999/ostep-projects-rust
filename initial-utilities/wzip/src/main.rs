use std::{env, fs::File, io::Read};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("wzip: file1 [file2 ...]");
        std::process::exit(1);
    }
    for filename in args.iter().skip(1) {
        let mut file = match File::open(filename) {
            Err(why) => panic!("couldn't open {}: {}", filename, why),
            Ok(f) => f,
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Err(why) => panic!("couldn't open {}: {}", filename, why),
            Ok(_) => {
                let char_arr: Vec<char> = contents.chars().collect();
                let mut prev_char = char_arr[0];
                let mut count = 1;
                for cur_char in char_arr.iter().skip(1) {
                    if prev_char != *cur_char {
                        print!("{prev_char}{count}");
                        prev_char = *cur_char;
                        count = 1;
                    } else {
                        count += 1;
                    }
                }
                print!("{prev_char}{count}");
                println!();
            },
        };
    }
}
