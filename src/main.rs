use std::env;
use crate::parser::*;
pub mod download;
pub mod parser;
extern crate termion;
#[macro_use]
pub mod output;

//------------- DEFAULTS -------------
const DEFAULT_FILE_FORMAT:FileFormat = FileFormat::Flac;
//------------- DEFAULTS -------------

pub enum FileFormat{
    Flac,
    Opus,
    Mp3,
    Wav,
    Vorbis,
    Aac
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


pub struct Options {
    bitrate:    Option<String>,
    format:     FileFormat,
    input_text: String,
    output_dir: String,
    songs:      Vec<SongDesc>
}

fn main() {
    is_sane();
    let args: Vec<String> = env::args().collect();
    let mut conf:Options = Options{
        format:     DEFAULT_FILE_FORMAT,
        input_text: String::new(),
        output_dir: String::new(),
        bitrate:    None,
        songs:      Vec::new()
    };

    let mut x = 1;
    while x < args.len(){match args[x].as_str(){
        "--help" | "-h" => {
            println!(include_str!("help.txt"));
            std::process::exit(0);
        },
        "-i" | "--input" => {
            x+=1;

            //check that the argument is complete.
            if x == args.len(){
                fatal!("fatal error: incomplete input argument.");
            }

            //read the file.
            conf.input_text+=&(safe_read_f(&args[x]) + "\n");
        },
        "-o" | "--output" => {
            x+=1;
            //check for a complete argument
            if x == args.len(){fatal!("fatal error: output directory arguemnt incomplete.")}

            safe_read_d(&args[x]); //check that the dir exists
            conf.output_dir = args[x].to_string();
        },
        "--format" | "-f" => {
            x+=1;
            if x == args.len(){fatal!("fatal error: format arguemnt incomplete.")}
            conf.format = match args[x].to_lowercase().trim(){
                "f" | "flac"    =>      FileFormat::Flac,
                "o" | "opus"    =>      FileFormat::Opus,
                "m" | "mp3"     =>      FileFormat::Mp3,
                "w" | "wav"     =>      FileFormat::Wav,
                "v" | "vorbis"  =>      FileFormat::Vorbis,
                "a" | "aac"     =>      FileFormat::Aac,
                _ => fatal!("fatal error: invalid format")
            }
        },
        _ => warn!(format!("warning: unknown argument: \"{}\"", args[x]))
        
    };x+=1}
    if conf.input_text.len() < 1{fatal!("fatal error: no input file(s) specified")}
    if conf.output_dir.len() < 1{fatal!("fatal error: no output directory specified")}

    parser::parse(&mut conf);
    download::download(conf);
}


use which::which;

fn is_sane(){
    let programs_needed = ["yt-dlp", "ffmpeg"];
    for i in programs_needed{
        match which(i){
            Ok(_) =>    (),
            Err(_) =>   fatal!(format!("Fatal error: {i} cannot be found"))
        }
    }
}

impl Options{

    //to be passed to ffmpeg
    pub fn bitrate(&self) -> String{
        match &self.bitrate {
            None => match self.format{
                FileFormat::Flac    => "0",
                FileFormat::Wav     => "0",
                FileFormat::Mp3     => "320k",
                FileFormat::Aac     => "256k",
                FileFormat::Vorbis  => "224k",
                FileFormat::Opus    => "192k"
            }.to_string(),
            Some(val) => val.clone()
        }
    }

    //again, for ffmpeg
    pub fn codec(&self) -> String{
        match self.format {
            FileFormat::Flac    => "flac",
            FileFormat::Wav     => "wav",
            FileFormat::Mp3     => "mp3",
            FileFormat::Aac     => "aac",
            FileFormat::Vorbis  => "libvorbis",
            FileFormat::Opus    => "libopus",
        }.to_string()
    }

    pub fn file_extension(&self) -> String{
        match self.format{
            FileFormat::Flac    => ".flac",
            FileFormat::Wav     => ".wav",
            FileFormat::Mp3     => ".mp3",
            FileFormat::Aac     => ".aac",
            FileFormat::Vorbis  => ".ogg",
            FileFormat::Opus    => ".ogg",
        }.to_string()
    }
}
