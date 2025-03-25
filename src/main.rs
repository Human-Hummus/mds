use crate::parser::*;
use crate::search::*;
use termion::color;
pub mod download;
pub mod logging;
pub mod parser;
pub mod search;
extern crate question;
extern crate reqwest;
extern crate termion;
use question::{Answer, Question};
use std::env;

//------------- DEFAULTS -------------
const DEFAULT_FILE_FORMAT: FileFormat = FileFormat::Flac;
//------------- DEFAULTS -------------

#[derive(Clone)]
pub enum FileFormat {
    Flac,
    Opus,
    Mp3,
    Wav,
    Vorbis,
    Aac,
}

#[derive(Clone)]
pub struct Options {
    bitrate: Option<String>,
    format: FileFormat,
    input_file: String,
    output_dir: String,
    songs: Vec<SongDesc>,
    clean_dir: bool,
}

fn main() {
    //use std::time::Instant;
    //0: only errors
    //1: only errors and warnings
    //2: all.
    let mut verbosity: u8 = 2;
    let mut searching = false; // is this a search? If so, we don't need an output file.
    let mut search_query = String::new();
    let mut get_artist = true;

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
                info!(include_str!("help.txt"));
                std::process::exit(0);
            }
            "-i" | "--input" => {
                x += 1;
                if x == args.len() {
                    fatal!("Incomplete input argument.");
                }

                //read the file.
                conf.input_file = args[x].clone()
            }
            "-o" | "--output" => {
                x += 1;
                if x == args.len() {
                    fatal!("Output directory arguemnt incomplete.")
                }

                if std::path::Path::new(&args[x]).exists() == false {
                    fatal!("Output dir does not exist.")
                }
                conf.output_dir = match &args[x].chars().nth(args[x].len() - 1).unwrap() {
                    '/' => args[x].clone(),
                    _ => args[x].clone() + "/",
                };
            }
            "--search" | "-s" => {
                x += 1;
                if x == args.len() {
                    fatal!("No search term provided.");
                }
                searching = true;
                search_query = args[x].to_lowercase();
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
            "--no-artist" | "-n" => get_artist = false,
            _ => warn!("Unknown argument: \"{}\"", (args[x])),
        };
        x += 1
    }
    if conf.input_file.len() < 1 {
        fatal!("No input file specified")
    }
    if conf.output_dir.len() < 1 && !searching {
        fatal!("No output directory specified")
    }
    //let mut now = Instant::now();
    conf.songs = parser::parse_file(&conf.input_file.to_string());
    if get_artist {
        parser::get_artist_name(&mut conf.songs)
    }
    //info!("Parser took {:?}", now.elapsed());
    //now = Instant::now();
    if searching {
        search_files(conf.songs, search_query);
        std::process::exit(0)
    }
    download::download(conf.clone(), verbosity);
    //info!("Downloader took {:?}", now.elapsed());
    //now = Instant::now();
    if conf.clean_dir {
        clean_directory(print_warning, conf);
        //info!("Cleaner took {:?}", now.elapsed());
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
                            panic!("Error Reading dir \"{}\"", conf.output_dir.clone())
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
        Err(_) => fatal!("Unknown file \"{}\"", conf.output_dir),
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
                fatal!("Music downloaded, but no files were deleted");
            }
            initial_warning = false;
            if Question::new(&format!(
                "\"{}\"({}) is not listed in the input file, delete it?",
                file.0, file.1
            ))
            .confirm()
                == Answer::NO
            {
                fatal!("File deletion aborted, {files_deleted} files deleted.")
            }
        }
        match std::fs::remove_file(&file.1) {
            Ok(_) => {
                info!("Deleted file \"{}\".", file.1);
                files_deleted += 1
            }
            Err(tp) => fatal!(
                "Failed to delete file \"{}\"({}). {files_deleted} files deleted. Error: {tp}",
                file.0,
                file.1
            ),
        };
    }
    info!(
        "{files_deleted} file{} deleted",
        match files_deleted {
            1 => " was",
            _ => "s were",
        }
    );
}

use which::which;

fn is_sane() {
    let programs_needed = ["yt-dlp", "ffmpeg", "wget"];
    for i in programs_needed {
        if which(i).is_err() {
            fatal!("Program {i} cannot be found")
        }
    }
}

impl Options {
    //to be passed to ffmpeg
    pub fn bitrate(&self) -> &str {
        match &self.bitrate {
            Some(value) => value,
            _ => match self.format {
                FileFormat::Flac => "0",
                FileFormat::Wav => "0",
                FileFormat::Mp3 => "320k",
                FileFormat::Aac => "256k",
                FileFormat::Vorbis => "224k",
                FileFormat::Opus => "192k",
            },
        }
    }

    //again, for ffmpeg
    pub fn codec(&self) -> &str {
        match self.format {
            FileFormat::Flac => "flac",
            FileFormat::Wav => "wav",
            FileFormat::Mp3 => "mp3",
            FileFormat::Aac => "aac",
            FileFormat::Vorbis => "libvorbis",
            FileFormat::Opus => "libopus",
        }
    }

    pub fn file_extension(&self) -> &str {
        match self.format {
            FileFormat::Flac => ".flac",
            FileFormat::Wav => ".wav",
            FileFormat::Mp3 => ".mp3",
            FileFormat::Aac => ".aac",
            FileFormat::Vorbis => ".ogg",
            FileFormat::Opus => ".ogg",
        }
    }
}
