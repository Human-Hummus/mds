use std::env;
mod download;
mod parser;
extern crate termion;
#[macro_use]
pub mod output;

pub enum FileFormat{
    flac,
    opus,
    mpeg,
    wav,
    vorbis,
    aac
}


fn safe_read_f(filepath: &String) -> String{
    return match std::fs::read_to_string(filepath){
        Ok(file) => {file},
        Err(_) => {fatal!(format!("fatal error: unknown file \"{}\"", filepath))}
    }
}

pub fn safe_read_d(dirpath: &String) -> Vec<String>{
    match std::fs::read_dir(dirpath){
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
    };
}

fn main() {
    is_sane();
    let args: Vec<String> = env::args().collect();
    let mut x = 1;
    let mut outtype = FileFormat::flac;
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
        }
        else if args[x] == "-o" || args[x] == "--output"{
            x+=1;
            //check for a complete argument
            if x == args.len(){fatal!("fatal error: output directory arguemnt incomplete.")}

            safe_read_d(&args[x]);
            output = args[x].to_string();
        }
        else if args[x] == "--format" || args[x] == "-f"{
            x+=1;
            if x == args.len(){fatal!("fatal error: format arguemnt incomplete.")}
            outtype = match args[x].to_lowercase().trim(){
                "f" | "flac" =>      FileFormat::flac,
                "o" | "opus" =>     FileFormat::opus,
                "m" | "mp3" =>      FileFormat::mpeg,
                "w" | "wav" =>      FileFormat::wav,
                "v" | "vorbis" =>   FileFormat::vorbis,
                "a" | "aac" =>      FileFormat::aac,
                _ => fatal!("fatal error: invalid format")
            }
        }
        else{
            warn!(format!("warning: unknown argument: \"{}\"", args[x]));
        }
        x+=1;
    }
    if text.len() < 1{fatal!("fatal error: no input file(s) specified")}
    if output.len() < 1{fatal!("fatal error: no output directory specified")}

    let stuff = parser::parse(&text);
    download::download(stuff, output, outtype);
}


use which::which;

fn is_sane(){
    match which("yt-dlp"){
        Ok(_) => (),
        Err(_) => fatal!("Fatal error: yt-dlp cannot be found")
    }
    match which("ffmpeg"){
        Ok(_) => (),
        Err(_) => fatal!("Fatal error: ffmpeg cannot be found")
    }
}

