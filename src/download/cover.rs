//this file is a mess.

extern crate log;
use log::{warn, info, trace};
use crate::download::gen_filename;
use std::fs;
use std::process::Command;

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
        let new_cover = wget_cover(&(og_cover.clone().unwrap()));
        trace!("{:?}", og_cover);
        match new_cover {
            Ok(nc) => {
                if verbosity > 1 {
                    trace!("Successfully downloaded cover art for \"{title}\" automatically!")
                }
                return Some(nc);
            },
            _ =>{
                trace!("Alert: unable to download cover for \"{}\"", title);
                return None
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
        if infile.contains("&"){
            warn!("Warning: song \"{title}\" has an input URL that contains additional information (detected by an ampersand). Consider removing this information as it may impair automatic thumbnail downloading.")
        }
        return youtube(infile, title);
    }
    if infile.contains("https://soundcloud.com") {
        return soundcloud(infile, title);
    }
    None
}

fn wget_cover(url: &String) -> Result<String, ()> {
    let newfilename = gen_filename(&"".to_string());
    return match std::process::Command::new("wget")
        .arg("-q")
        .arg(url)
        .arg("-O")
        .arg(newfilename.clone())
        .status()
    {
        Ok(k) => match k.success() {
            true => Ok(newfilename),
            false => Err(()),
        },
        Err(_) => Err(()),
    };
}

fn youtube(infile: &String, title: &String) -> Option<String> {
    let mut the_vid_link = String::new();
    if infile.contains("https://youtube.com") || infile.contains("https://www.youtube.com") {
        the_vid_link = infile
            .split("https://www.youtube.com/watch?v=")
            .collect::<Vec<&str>>()[1]
            .to_owned();
    }
    if infile.contains("https://youtu.be") {
        the_vid_link = infile.split("https://www.youtu.be/").collect::<Vec<&str>>()[1].to_owned();
    }
    match wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &the_vid_link + "/hqdefault.jpg")){
        Ok(tr) => return Some(tr),
        _ => {}
    }
    match wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &the_vid_link + "/hq720.jpg")){
        Ok(tr) => return Some(tr),
        _ => {}
    }

    info!("Failed to automatically download cover art for \"{}\". This can be ignored.",title);
    None
}

fn soundcloud(infile: &String, title: &String) -> Option<String> {
    let sc_html_file = wget_cover(infile);
    if sc_html_file.is_err(){
        trace!("Error downloading cover from soundcloud for \"{title}\"");
        return None
    }
    let contents = fs::read_to_string(sc_html_file.clone().unwrap()).unwrap();
    if !contents.contains("src=\"https://i1.sndcdn.com") {
        Command::new("rm")
            .arg("-f")
            .arg(sc_html_file.unwrap())
            .status()
            .expect("This error shouln't be possible...");
        trace!("Error downloading cover from soundcloud for \"{title}\"");
        return None;
    }

    //I know, this isn't good; I've been a bad boy. (UwU)
    let img_link = format!(
        "https://i1.sndcdn.com{}",
        contents
            .split("src=\"https://i1.sndcdn.com")
            .collect::<Vec<&str>>()[1]
            .to_string()
            .split("\"")
            .collect::<Vec<&str>>()[0]
            .trim()
            .to_owned()
    );

    trace!("Soundcloud cover art function: img_link: {img_link}");
    Command::new("rm")
        .arg("-f")
        .arg(sc_html_file.unwrap())
        .status()
        .expect("This error shouln't be possible...");
    match wget_cover(&img_link) {
        Err(_) => {
            trace!(
                "(2)Error downloading soundcloud cover art for \"{}\"",
                title
            );
            return None;
        }
        Ok(x) => {
            return Some(x);
        }
    }
}
