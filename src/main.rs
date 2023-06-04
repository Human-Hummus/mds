use std::env;
mod download;
mod parser;
extern crate termion;
use termion::color::Fg;
use termion::{color};

const GREEN: Fg<color::Green> = color::Fg(color::Green);
const YELLOW: Fg<color::Yellow> = color::Fg(color::Yellow);
const RED: Fg<color::Red> = color::Fg(color::Red);
const CLR: Fg<color::Reset> = color::Fg(color::Reset);


fn safe_read_f(filepath: &String) -> String{
    return match std::fs::read_to_string(filepath){
        Ok(file) => {file},
        Err(_) => {eprintln!("fatal error: unknown file \"{}\"", filepath); std::process::exit(1);}
    }
}
fn safe_read_d(dirpath: &String) -> Vec<String>{
    return match std::fs::read_dir(dirpath){
        Ok(file) => {
                let mut out:Vec<String> = [].to_vec();
                for i in file{
                    out.push(match i{
                        Ok(path) => {path.path()}
                        Err(_) => {eprintln!("fatal error: error reading directory \"{}\"", dirpath);std::process::exit(1);}
                    }.display().to_string());
                }
                return out;
            },
        Err(_) => {eprintln!("fatal error: unknown file \"{}\"", dirpath); std::process::exit(1);}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut x = 1;
    let mut outtype = 'f'; // can be (F)lac, (O)pus, (M)p3, (W)av, (V)orbis, or (A)ac. always lowercase.
    let mut text:String = String::new();
    let mut output = String::new();

    while x < args.len(){
        if args[x] == "--help" || args[x] == "-h"{
            println!("{}", include_str!("help.txt"));
            std::process::exit(0);
        }
        else if args[x] == "-i" || args[x] == "--input"{
            x+=1;

            //check that the argument is complete.
            if x == args.len(){
                println!("{}fatal error: incomplete input argument.{}",RED,CLR);
                std::process::exit(1);
            }

            //read the file.
            text+=&(safe_read_f(&args[x]) + "\n");
            x+=1;
        }
        else if args[x] == "-o" || args[x] == "--output"{
            x+=1;
            //check for a complete argument
            if x == args.len(){println!("{}fatal error: output directory arguemnt incomplete.{}",RED,CLR);std::process::exit(1);}

            safe_read_d(&args[x]);
            output = args[x].to_string();
            x+=1;
        }
        else if args[x] == "--format" || args[x] == "-f"{
            if x == args.len(){println!("{}fatal error: output directory arguemnt incomplete.{}",RED,CLR);std::process::exit(1);}
            x+=1;
            let axl = args[x].to_lowercase().trim().to_owned();
            println!("argxlow: {}", axl);
            if axl == "f"|| axl == "flac"{
                outtype = 'f';
            }
            else if axl == "o" || axl == "opus"{
                outtype = 'o'
            }
            else if axl == "m" || axl == "mp3"{
                outtype = 'm'
            }
            else if axl == "w" || axl == "wav"{
                outtype = 'w'
            }
            else if axl == "v" || axl == "vorbis"{
                outtype = 'v'
            }
            else if axl == "a" || axl == "aac"{
                outtype = 'a'
            }
            else{println!("{}fatal error: invalid format{}",RED,CLR);}
            x+=1;
        }
        else{
            println!("{}warning: unknown argument: \"{}\"{}", YELLOW, args[x], CLR);
            x+=1;
        }
    }
    if text.len() < 1{println!("{}fatal error: no input file(s) specified{}", RED,CLR);std::process::exit(1)}
    if output.len() < 1{println!("{}fatal error: no output directory specified{}", RED,CLR);std::process::exit(1);}
    
    let stuff = parser::parse(&text);
    download::download(stuff, output, outtype);


}
