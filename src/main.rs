use crate::parser::*;
use std::env;
pub mod download;
pub mod parser;
extern crate question;
extern crate termion;
use question::Answer;
use question::Question;
#[macro_use]
pub mod output;

//------------- DEFAULTS -------------
const DEFAULT_FILE_FORMAT: FileFormat = FileFormat::Flac;
//------------- DEFAULTS -------------

pub enum FileFormat {
    Flac,
    Opus,
    Mp3,
    Wav,
    Vorbis,
    Aac,
}
impl FileFormat {
    pub fn clone(&self) -> FileFormat {
        match self {
            FileFormat::Flac => FileFormat::Flac,
            FileFormat::Opus => FileFormat::Opus,
            FileFormat::Mp3 => FileFormat::Mp3,
            FileFormat::Wav => FileFormat::Wav,
            FileFormat::Vorbis => FileFormat::Vorbis,
            FileFormat::Aac => FileFormat::Aac,
        }
    }
}
pub struct Options {
    bitrate: Option<String>,
    format: FileFormat,
    input_file: String,
    output_dir: String,
    songs: Vec<SongDesc>,
    clean_dir: bool,
}

fn main() {
    //0: only errors
    //1: only errors and warnings
    //2: all.
    let mut verbosity: u8 = 2;
    is_sane();
    let args: Vec<String> = env::args().collect();
    let mut conf: Options = Options {
        format: DEFAULT_FILE_FORMAT,
        input_file: String::new(),
        output_dir: String::new(),
        bitrate: None,
        songs: Vec::new(),
        clean_dir: false,
    };
    let mut print_warning = true;

    let mut x = 1;
    while x < args.len() {
        match args[x].as_str() {
            "--help" | "-h" => {
                println!(include_str!("help.txt"));
                std::process::exit(0);
            }
            "-i" | "--input" => {
                x += 1;

                //check that the argument is complete.
                if x == args.len() {
                    fatal!("fatal error: incomplete input argument.");
                }

                //read the file.
                conf.input_file = args[x].clone()
            }
            "-o" | "--output" => {
                x += 1;
                //check for a complete argument
                if x == args.len() {
                    fatal!("fatal error: output directory arguemnt incomplete.")
                }

                if std::path::Path::new(&args[x]).exists() == false {
                    fatal!("Fatal error: output dir does not exist.")
                }
                conf.output_dir = match &args[x].chars().nth(args[x].len() - 1).unwrap() {
                    '/' => args[x].clone(),
                    _ => args[x].clone() + "/",
                };
            }
            "--format" | "-f" => {
                x += 1;
                if x == args.len() {
                    fatal!("fatal error: format arguemnt incomplete.")
                }
                conf.format = match args[x].to_lowercase().trim() {
                    "f" | "flac" => FileFormat::Flac,
                    "o" | "opus" => FileFormat::Opus,
                    "m" | "mp3" => FileFormat::Mp3,
                    "w" | "wav" => FileFormat::Wav,
                    "v" | "vorbis" => FileFormat::Vorbis,
                    "a" | "aac" => FileFormat::Aac,
                    _ => fatal!("fatal error: invalid format"),
                }
            }
            "--do-not-warn" => print_warning = false,
            "--delete-not-present" => conf.clean_dir = true,
            "--verbose" | "-v" => verbosity = 2,
            "--quiet" | "-q" => verbosity = 1,
            "--silent" | "-Q" => verbosity = 0,
            _ => warn!(format!("warning: unknown argument: \"{}\"", args[x])),
        };
        x += 1
    }
    if conf.input_file.len() < 1 {
        fatal!("fatal error: no input file specified")
    }
    if conf.output_dir.len() < 1 {
        fatal!("fatal error: no output directory specified")
    }

    conf.songs = parser::parse_file(&conf.input_file.to_string());
    debug!(format!("full parser output: {:?}\n\n\n", conf.songs));
    download::download(
        match conf.clean_dir {
            true => conf.clone(),
            false => conf.clone(),
        },
        verbosity,
    );
    if conf.clean_dir {
        clean_directory(print_warning, conf);
    }
}

fn clean_directory(warning: bool, conf: Options) {
    let mut files_deleted = 0;
    let mut initial_warning = warning;
    let files: Vec<(String, String)> = match std::fs::read_dir(&conf.output_dir) {
        Ok(file) => {
            let mut out: Vec<(String, String)> = [].to_vec();
            for i in file {
                out.push((
                    (&i.as_ref()
                        .unwrap_or_else(|_| {
                            fatal!(format!("Error Reading dir \"{}\"", conf.output_dir.clone()))
                        })
                        .path()
                        .file_stem()
                        .expect("failed on directory")
                        .to_string_lossy()
                        .into_owned())
                        .clone(),
                    i.expect("").path().into_os_string().into_string().unwrap(),
                ))
            }
            out
        }
        Err(_) => fatal!(format!("fatal error: unknown file \"{}\"", conf.output_dir)),
    };

    'bigloop: for file in files {
        for song in &conf.songs {
            if song.name == file.0 {
                continue 'bigloop;
            }
        }
        if warning {
            if initial_warning
                && Question::new("Are you sure you want to remove unlisted files?").confirm()
                    == Answer::NO
            {
                fatal!("Fatal Error: Music downloaded, but no files were deleted");
            }
            initial_warning = false;
            if Question::new(
                format!(
                    "\"{}\"({}) is not listed in the input file, delete it?",
                    file.0, file.1
                )
                .as_str(),
            )
            .confirm()
                == Answer::NO
            {
                fatal!("fatal: file deletion aborted, {files_deleted} files deleted.")
            }
        }
        match std::fs::remove_file(&file.1){
            Ok(_) => {alert!(format!("Deleted file \"{}\".",file.1));files_deleted+=1},
            Err(tp)=> fatal!(format!("Fatal: Failed to delete file \"{}\"({}). {files_deleted} files deleted. Error: {tp}",file.0, file.1))
        };
    }
    alert!(format!(
        "{files_deleted} file{} {} deleted",
        match files_deleted {
            1 => "",
            _ => "s",
        },
        match files_deleted {
            1 => "was",
            _ => "were",
        }
    ));
}

use which::which;

fn is_sane() {
    let programs_needed = ["yt-dlp", "ffmpeg"];
    for i in programs_needed {
        if which(i).is_err() {
            fatal!(format!("Fatal error: {i} cannot be found"))
        }
    }
}

impl Options {
    //to be passed to ffmpeg
    pub fn bitrate(&self) -> String {
        match &self.bitrate {
            None => match self.format {
                FileFormat::Flac => "0",
                FileFormat::Wav => "0",
                FileFormat::Mp3 => "320k",
                FileFormat::Aac => "256k",
                FileFormat::Vorbis => "224k",
                FileFormat::Opus => "192k",
            }
            .to_string(),
            Some(val) => val.clone(),
        }
    }

    pub fn clone(&self) -> Options {
        return Options {
            bitrate: self.bitrate.clone(),
            format: self.format.clone(),
            input_file: self.input_file.clone(),
            output_dir: self.output_dir.clone(),
            songs: self.songs.clone(),
            clean_dir: self.clean_dir,
        };
    }

    //again, for ffmpeg
    pub fn codec(&self) -> String {
        match self.format {
            FileFormat::Flac => "flac",
            FileFormat::Wav => "wav",
            FileFormat::Mp3 => "mp3",
            FileFormat::Aac => "aac",
            FileFormat::Vorbis => "libvorbis",
            FileFormat::Opus => "libopus",
        }
        .to_string()
    }

    pub fn file_extension(&self) -> String {
        match self.format {
            FileFormat::Flac => ".flac",
            FileFormat::Wav => ".wav",
            FileFormat::Mp3 => ".mp3",
            FileFormat::Aac => ".aac",
            FileFormat::Vorbis => ".ogg",
            FileFormat::Opus => ".ogg",
        }
        .to_string()
    }
}
