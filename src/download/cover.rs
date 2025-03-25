//this file is a mess.
use crate::download::gen_filename;
use crate::*;
use std::fs;
use std::io::Write;

//the output is a file path OR None.
pub fn process_cover(
    og_cover: &Option<String>,
    is_url: bool,
    is_infile_link: bool,
    infile: &String,
    title: &String,
    verbosity: u8,
) -> Option<String> {
    if is_url {
        let new_cover = get_cover(&(og_cover.clone().unwrap()));
        match new_cover {
            Ok(nc) => {
                if verbosity > 1 {
                    info!("Successfully downloaded cover art for \"{title}\" automatically!")
                }
                return Some(nc);
            }
            _ => {
                trivial!("Alert: unable to download cover for \"{}\"", title);
                return None;
            }
        };
    } else if og_cover.is_none() && is_infile_link {
        return download_cover_art(infile, title);
    } else {
        return og_cover.clone();
    }
}

fn download_cover_art(infile: &String, title: &String) -> Option<String> {
    if infile.contains("https://www.youtube.com")
        || infile.contains("https://youtube.com")
        || infile.contains("https:/youtu.be")
    {
        if infile.contains("&") {
            fatal!("Song \"{title}\" has a URL that contains additional information (detected by an ampersand). Remove this information as it may impair automatic downloading.")
        }
        return youtube(infile, title);
    }
    if infile.contains("https://soundcloud.com") {
        return soundcloud(infile, title);
    }
    None
}
macro_rules! err_to_empty_err {
    ($x:expr) => {
        match $x {
            Ok(value) => Ok(value),
            Err(_) => return Err(()),
        }
    };
}

fn get_cover(url: &String) -> Result<String, ()> {
    let runtime = err_to_empty_err!(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build())?;
    let filename = gen_filename("");
    let content = err_to_empty_err!(
        runtime.block_on(err_to_empty_err!(runtime.block_on(reqwest::get(url)))?.bytes())
    )?;
    err_to_empty_err!(err_to_empty_err!(fs::File::create(&filename))?.write_all(&content))?;
    Ok(filename)
}

fn youtube(infile: &String, title: &String) -> Option<String> {
    let mut the_vid_link = String::new();
    if infile.contains("https://youtube.com") || infile.contains("https://www.youtube.com") {
        the_vid_link = infile
            .split("https://www.youtube.com/watch?v=")
            .nth(1)?
            .to_owned();
    }
    if infile.contains("https://youtu.be") {
        the_vid_link = infile.split("https://www.youtu.be/").nth(1)?.to_owned();
    }
    match get_cover(&format!(
        "https://i.ytimg.com/vi/{the_vid_link}/hqdefault.jpg"
    )) {
        Ok(tr) => return Some(tr),
        _ => {}
    }
    match get_cover(&format!("https://i.ytimg.com/vi/{the_vid_link}/hq720.jpg")) {
        Ok(tr) => return Some(tr),
        _ => {}
    }

    info!("Failed to automatically download cover art for \"{title}\". This can be ignored.");
    None
}

fn soundcloud(infile: &String, title: &String) -> Option<String> {
    let soundcloud_html_file = match get_cover(infile) {
        Ok(value) => value,
        Err(_) => {
            warn!("Error downloading cover from soundcloud for \"{title}\"");
            return None;
        }
    };
    let contents = fs::read_to_string(soundcloud_html_file.clone()).unwrap();
    if !contents.contains("src=\"https://i1.sndcdn.com") {
        fs::remove_file(soundcloud_html_file).unwrap();
        warn!("Error downloading cover from soundcloud for \"{title}\"");
        return None;
    }

    //I know, this isn't good; I've been a bad boy. (UwU)
    let img_link = format!(
        "https://i1.sndcdn.com{}",
        contents
            .split("src=\"https://i1.sndcdn.com")
            .nth(1)?
            .split("\"")
            .nth(0)?
            .trim()
            .to_owned()
    );
    fs::remove_file(soundcloud_html_file).unwrap();
    match get_cover(&img_link) {
        Err(_) => {
            warn!("(2)Error downloading soundcloud cover art for \"{title}\"");
            None
        }
        Ok(value) => Some(value),
    }
}
