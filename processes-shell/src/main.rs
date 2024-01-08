use std::{env, io::{self, Write, BufRead}, process::Command, path::Path, fs::File};

const SHELL_START: &str = "wish> ";
const EXIT_COMMAND: &str = "exit";
const DEFAULT_PATH: &str = "/bin/";
const CHANGE_DIRECTORY: &str = "cd";
const PATH: &str = "path";

struct State {
    end: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!("too many arguments")
    }else if args.len() == 2 {
        batch_mode(&args[1])
    }else {
        interactive_mode()
    }
}

fn interactive_mode(){
    let mut line_buffer: String;
    let mut paths :Vec<String> = vec![DEFAULT_PATH.to_string()];
    loop {
        print!("{}", SHELL_START);
        io::stdout().flush().unwrap();
        line_buffer = String::new();
        io::stdin().read_line(&mut line_buffer).unwrap();
        
        match execute_command(line_buffer, &mut paths) {
            Ok(s) => if s.end {
                break
            },
            Err(e) => panic!("{}",e),
        }
    }
}

fn batch_mode(filename: &String){
    let mut paths :Vec<String> = vec![DEFAULT_PATH.to_string()];
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match execute_command(line.unwrap(), &mut paths) {
            Ok(s) => if s.end {
                break
            },
            Err(e) => panic!("{}",e),
        }
    }
}

fn execute_command(line: String, paths: &mut Vec<String>) -> Result<State, io::Error> {
    let mut args = line.trim().split_whitespace();
    let mut command = args.next().unwrap().to_string();
        match command.as_str() {
            EXIT_COMMAND => Ok(State {end: true}),
            CHANGE_DIRECTORY => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                match env::set_current_dir(&root) {
                    Ok(_) => Ok(State {end: false}),
                    Err(e) => Err(e),
                }
            },
            PATH => {
                let temp: Vec<String> = args.map(|x| {
                    let mut result = x.to_string();
                    if !x.ends_with("/") {
                        result.push('/');
                    }
                    result
                }).collect();
                paths.clear();
                paths.extend(temp);
                Ok(State {end: false})
            }
            _ => {
                // check this arguments need path
                if !(command.starts_with("/") || command.starts_with(".")) {
                    // search binary file in paths
                    for path in paths {
                        command = format!("{path}{command}");
                        if Path::new(&command).exists() {
                            break;
                        }
                    }
                }

                if !Path::new(&command).exists() {
                    println!("command are not exist");
                    return Ok(State{end: false});
                }
                
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut c) => {
                        let _ = c.wait();
                        Ok(State{end: false})
                    },
                    Err(e) => panic!("{}",e),
                }
            },
        }
}