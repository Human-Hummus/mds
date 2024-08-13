use delete;
use rand;
use rand::distributions::{Distribution, Uniform};
extern crate log;
use log::{info, trace, error};
mod cover;
use crate::*;

pub fn download(conf: Options, verbosity: u8) {
    let files: Vec<String> = match std::fs::read_dir(&conf.output_dir) {
        Ok(file) => {
            let mut out: Vec<String> = [].to_vec();
            for i in file {
                out.push(
                    i.unwrap_or_else(|_| {
                        panic!("Error Reading dir \"{}\"", conf.output_dir)
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
        Err(_) => panic!("fatal error: unknown file \"{}\"", conf.output_dir),
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
            trace!("file \"{}\" is already present.", song.name);
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
            trace!("tdx {:?}", song);
            infile = match tmp_ytdlp(&song.infile) {
                None => {
                    errored += 1.0;
                    file_errors += &format!("\n\t* \"{}\"", song.name);
                    error!("Error while downloading \"{}\".", song.name);
                    x += 1;
                    continue;
                }
                Some(val) => val,
            }
        } else {
            infile = song.infile.clone();
        }

        let outfile: String = format!(
            "{}{}{}",
            &match &conf
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

        match final_ffmpeg(&song.cover, &outfile, &infile, &conf, &song.artist) {
            Some(_) => (),
            None => {
                error!(
                    "Non-fatal error: failed to convert \"{}\"",
                    song.name
                );
                x += 1;
                continue;
            }
        };
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
            100.0 * (total_files_already_present / total_songs_seen)
        );
        info!(
            "Total files failed: {:.0}({:.0}%)",
            errored,
            100.0 * (errored / total_files_already_present)
        );
    }
    if file_errors != String::new() {
        error!("List of files failed:{file_errors}")
    }
}

fn final_ffmpeg(
    cover: &Option<String>,
    outputfile: &String,
    infile: &String,
    conf: &Options,
    artist: &String
) -> Option<i8> {
    match cover{
        None =>{
            let mut cmd = std::process::Command::new("ffmpeg");
            cmd.arg("-i")
            .arg(infile.trim().to_owned())
            .arg("-c:a")
            .arg(conf.codec())
            .arg("-b:a")
            .arg(conf.bitrate())
            .arg("-loglevel")
            .arg("error");

            if artist != ""{
                cmd.arg("-metadata:s:v").arg(format!("title=\"{}\"", str::replace(artist, "\"", "\\\"")));
            }
            cmd.arg(outputfile);

            cmd.status().ok()?},
    
        Some(cvr) => {
            let mut cmd = std::process::Command::new("ffmpeg");

            cmd.arg("-i")
            .arg(infile.trim().to_owned())
            .arg("-i")
            .arg(cvr.trim().to_owned())
            .arg("-map")
            .arg("0:a")
            .arg("-map")
            .arg("1:v")
            .arg("-c:a")
            .arg(conf.codec())
            .arg("-b:a")
            .arg(conf.bitrate())
            .arg("-disposition:1")
            .arg("attached_pic")
            .arg("-loglevel")
            .arg("error");
            if artist != ""{
                cmd.arg("-metadata:s:v").arg(format!("title={}", str::replace(artist, "\"", "\\\"")));
            }
            cmd.arg(outputfile);

            cmd.status().ok()?}
    };
    Some(0)
}

#[inline(always)]
pub fn gen_filename(fex: &String) -> String {
    let mut length = 100;
    let chars_allowed = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890-_"
        .chars()
        .collect::<Vec<char>>();
    let mut fname = "/tmp/".to_string();
    length -= fname.len() + fex.len();

    while length != 0 {
        length -= 1;
        fname.push(
            chars_allowed
                [Uniform::from(0..chars_allowed.len() - 1).sample(&mut rand::thread_rng())],
        );
    }
    return fname.to_owned() + fex;
}

fn tmp_ytdlp(url: &String) -> Option<String> {
    let fname = gen_filename(&".flac".to_owned());
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

#[inline(always)]
fn is_done(title: &String, files: &Vec<String>) -> bool {
    match files.binary_search(title) {
        Ok(_) => true,
        _ => false,
    }
}
