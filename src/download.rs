use delete;
use rand;
use rand::prelude::IteratorRandom;
mod cover;
use crate::*;
use termion::color;

pub fn download(conf: Options, verbosity: u8) {
    let files: Vec<String> = match std::fs::read_dir(&conf.output_dir) {
        Ok(file) => {
            let mut out: Vec<String> = Vec::new();
            for i in file {
                out.push(
                    i.unwrap_or_else(|_| {
                        fatal!("Failed to read directory \"{}\"", conf.output_dir);
                    })
                    .path()
                    .file_stem()
                    .expect("failed on directory")
                    .to_string_lossy()
                    .into_owned(),
                )
            }
            out.sort();
            out
        }
        Err(_) => fatal!("Unknown directory \"{}\"", conf.output_dir),
    };
    let (
        mut total_files_already_present,
        mut total_songs_seen,
        mut errored,
        mut x,
        mut file_errors,
    ) = (0.0, 0.0, 0.0, 0, String::new());
    while x < conf.songs.len() {
        let mut song = conf.songs[x].clone();
        total_songs_seen += 1.0;
        let infile: String;

        if is_done(&song.name, &files) {
            if verbosity <= 1 {
                trivial!("file \"{}\" is already present.", song.name);
            }
            total_files_already_present += 1.0;
            x += 1;
            continue;
        }
        if verbosity > 1 {
            info!(
                "{} song \"{}\" to output directory.",
                match song.is_file_url {
                    true => "Downloading and copying",
                    false => "Copying",
                },
                song.name
            )
        };

        song.cover = cover::process_cover(
            &song.cover,
            song.is_cover_url,
            song.is_file_url,
            &song.infile,
            &song.name,
            verbosity,
        );

        if song.is_file_url {
            infile = match tmp_ytdlp(&song.infile) {
                Some(value) => value,
                _ => {
                    errored += 1.0;
                    file_errors += &format!("\n\t* \"{}\"", song.name);
                    error!("Error while downloading \"{}\".", song.name);
                    x += 1;
                    continue;
                }
            }
        } else {
            infile = song.infile.clone();
        }

        let outfile: String = format!(
            "{}{}{}",
            match &conf
                .output_dir
                .chars()
                .nth(conf.output_dir.len() - 1)
                .unwrap()
            {
                '/' => conf.output_dir.clone(),
                _ => conf.output_dir.clone() + "/",
            },
            song.name,
            conf.file_extension()
        );
        // if final_ffmpeg fails
        if !final_ffmpeg(&song.cover, &outfile, &infile, &conf, &song.artist) {
            error!("Failed to convert \"{}\"", song.name);
            x += 1;
            continue;
        }
        if song.is_file_url {
            delete::delete_file(&infile).unwrap();
        }
        if song.is_cover_url && !song.cover.is_none() {
            delete::delete_file(&song.cover.unwrap()).unwrap();
        }
        x += 1;
    }
    if verbosity > 1 {
        info!("Total files listed: {:.0}", total_songs_seen);
        info!(
            "Total files already present: {:.0}({:.1}%).",
            total_files_already_present,
            match ((total_files_already_present / total_songs_seen) as f32).is_nan() {
                true => 100.0,
                _ => 100.0 * (total_files_already_present / total_songs_seen),
            }
        );
        info!(
            "Total files failed: {:.0}({:.0}%)",
            errored,
            match ((errored / total_files_already_present) as f32).is_nan() {
                true => 0.0,
                _ => 100.0 * (errored / total_files_already_present),
            }
        );
    }
    if file_errors != "" {
        error!("List of files failed:\n{file_errors}");
    }
}

fn final_ffmpeg(
    cover: &Option<String>,
    outputfile: &String,
    infile: &String,
    conf: &Options,
    artist: &String,
) -> bool {
    // succeeded?
    let mut cmd = std::process::Command::new("ffmpeg");
    cmd.arg("-i").arg(infile.trim());
    if cover.is_some() {
        cmd.arg("-i")
            .arg(cover.clone().unwrap().trim())
            .arg("-map")
            .arg("0:a")
            .arg("-map")
            .arg("1:v")
            .arg("-disposition:1")
            .arg("attached_pic");
    }
    cmd.arg("-c:a")
        .arg(conf.codec())
        .arg("-b:a")
        .arg(conf.bitrate())
        .arg("-loglevel")
        .arg("error");
    if artist != "" {
        cmd.arg("-metadata:s:v").arg(format!("title=\"{artist}\""));
    }
    match cmd.arg(outputfile).status().ok() {
        Some(value) => value.success(),
        _ => false,
    }
}

pub fn gen_filename(fex: &str) -> String {
    let mut rng = rand::thread_rng();
    let chars_allowed = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890-_";
    let mut fname = "/tmp/".to_string();
    for _ in 0..100 {
        fname.push(chars_allowed.chars().choose(&mut rng).unwrap())
    }
    fname + fex
}

fn tmp_ytdlp(url: &String) -> Option<String> {
    let fname = gen_filename(".flac");
    match std::process::Command::new("yt-dlp")
        .arg(url)
        .arg("-x")
        .arg("--audio-format")
        .arg("flac")
        .arg("-q")
        .arg("--progress")
        .arg("-o")
        .arg(fname.clone())
        .status()
        .unwrap()
        .success()
    {
        true => Some(fname),
        false => None,
    }
}

fn is_done(title: &String, files: &Vec<String>) -> bool {
    files.binary_search(title).is_ok()
}
