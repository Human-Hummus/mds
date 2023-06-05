use std::env;
mod download;
mod parser;
extern crate termion;
#[macro_use]
pub mod output;


fn safe_read_f(filepath: &String) -> String{
    return match std::fs::read_to_string(filepath){
        Ok(file) => {file},
        Err(_) => {fatal!(format!("fatal error: unknown file \"{}\"", filepath))}
    }
}
fn safe_read_d(dirpath: &String) -> Vec<String>{
    return match std::fs::read_dir(dirpath){
        Ok(file) => {
                let mut out:Vec<String> = [].to_vec();
                for i in file{
                    out.push(match i{
                        Ok(path) => {path.path()}
                        Err(_) => {fatal!(format!("fatal error: error reading directory \"{}\"", dirpath))}
                    }.display().to_string());
                }
                return out;
            },
        Err(_) => fatal!(format!("fatal error: unknown file \"{}\"", dirpath))
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
                fatal!("fatal error: incomplete input argument.");
            }

            //read the file.
            text+=&(safe_read_f(&args[x]) + "\n");
            x+=1;
        }
        else if args[x] == "-o" || args[x] == "--output"{
            x+=1;
            //check for a complete argument
            if x == args.len(){fatal!("fatal error: output directory arguemnt incomplete.")}

            safe_read_d(&args[x]);
            output = args[x].to_string();
            x+=1;
        }
        else if args[x] == "--format" || args[x] == "-f"{
            if x == args.len(){fatal!("fatal error: output directory arguemnt incomplete.")}
            x+=1;
            let axl = args[x].to_lowercase().trim().to_owned();
            debug!(format!("argxlow: {}", axl));
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
            else{fatal!("fatal error: invalid format")}
            x+=1;
        }
        else{
            warn!(format!("warning: unknown argument: \"{}\"", args[x]));
            x+=1;
        }
    }
    if text.len() < 1{fatal!("fatal error: no input file(s) specified")}
    if output.len() < 1{fatal!("fatal error: no output directory specified")}
    
    let stuff = parser::parse(&text);
    download::download(stuff, output, outtype);


}
