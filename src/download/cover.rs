//this file is a mess.

use crate::download::gen_filename;
use crate::*;
use std::fs;
use std::process::Command;

//the output is a filename OR "None".
pub fn process_cover(og_cover: &String, is_url: bool, is_infile_link: bool, infile: &String, title: &String) -> String{
    if is_url{
        let new_cover = wget_cover(og_cover);
        if new_cover == "ERR"{
            debug!(format!("Alert: unable to download cover for \"{}\"", title));
            return String::from("None");
        }
        alert!(format!("Successfully downloaded cover art for \"{}\" automatically!",title));
        return new_cover;
    }
    else if og_cover == "None" && is_infile_link{
        return download_cover_art(infile, title);
    }
    else{
        return (og_cover).to_string();
    }
}


fn download_cover_art(infile: &String, title: &String) -> String{
    if infile.contains("https://www.youtube.com") || infile.contains("https://youtube.com") || infile.contains("https:/youtu.be"){return youtube(infile,title)}
    if infile.contains("https://soundcloud.com"){return soundcloud(infile, title)}
    return "None".to_string();
}

fn wget_cover(url:&String)->String{
    let newfilename = gen_filename(&"".to_string());
    return match 
    std::process::Command::new("wget").arg("-q").arg(url).arg("-O").arg(newfilename.clone()).status(){
        Ok(k) => match k.success(){
            true => newfilename,
            false => "ERR".to_string()
        },
        Err(_) => "ERR".to_string()

    };
}




fn youtube(infile:&String, title:&String) -> String{
    if infile.contains("https://youtube.com") || infile.contains("https://www.youtube.com"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtube.com/watch?v=").collect::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        debug!(format!("Failed to automatically download cover art for \"{}\". This can be ignored.", title));
        return "None".to_string();
    }
    if infile.contains("https://youtu.be"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtu.be/").collect::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        debug!(format!("Failed to automatically download cover art for \"{}\". This can be ignored.", title));
        return "None".to_string();
    }
    return "None".to_string();
}

fn soundcloud(infile: &String, title:&String) -> String{
    let sc_html_file = wget_cover(infile);
    if sc_html_file == "ERR"{debug!(format!("(0)error downloading cover from soundcloud for \"{}\"", title));return "None".to_string();}
    let contents = fs::read_to_string(sc_html_file.clone()).expect("error; this shouldn't happen...");
    if !(contents.contains("src=\"https://i1.sndcdn.com")){
        Command::new("rm").arg("-f").arg(sc_html_file.clone()).status().expect("This error shouln't be possible...");
        debug!(format!("(1)error downloading cover from soundcloud for \"{}\"",title));
        return "None".to_string();
    }

    //I know, this isn't good; I've been a bad boy.
    let img_link = format!("https://i1.sndcdn.com{}", contents.split("src=\"https://i1.sndcdn.com").collect::<Vec<&str>>()[1].to_string().split("\"").collect::<Vec<&str>>()[0].trim().to_owned());
    
    debug!(format!("soundcloud cover art function: img_link: {}", img_link));
    Command::new("rm").arg("-f").arg(sc_html_file.clone()).status().expect("This error shouln't be possible...");
    match wget_cover(&img_link).as_str(){
        "ERR" => {debug!(format!("(2)Error downloading soundcloud cover art for \"{}\"", title));return "None".to_string()},
        x => {return x.to_string();}

    }

}
