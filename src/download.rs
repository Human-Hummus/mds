use std::fs;
use rand;
extern crate termion;
use crate::parser::SongDesc;
use rand::distributions::Uniform;
use termion::color::Fg;
use rand::distributions::Distribution;
use termion::{color};

const GREEN: Fg<color::Green> = color::Fg(color::Green);
const YELLOW: Fg<color::Yellow> = color::Fg(color::Yellow);
const RED: Fg<color::Red> = color::Fg(color::Red);
const CLR: Fg<color::Reset> = color::Fg(color::Reset);

pub fn download(todo: Vec<SongDesc>, outdir: String, fex: String){
    let mut x = 0;
    while x < todo.len(){
        let infile:String;
        let cover:String;
        let title = todo[x].name.clone();
        if is_done(&title, &outdir, &fex){println!("{}file \"{}\" is already present.{}", GREEN, title, CLR);x+=1;continue;}
        if todo[x].is_file_url{
            infile = tmp_ytdlp(&todo[x].infile, &fex);
            if infile == "ERR"{
                println!("{}Fatal error while downloading \"{}\".{}", RED, title, CLR);
                x+=1;
                continue;
            }
        }
        else {infile = todo[x].infile.clone();}
        if todo[x].is_cover_url{
            cover = wget_cover(&todo[x].cover);
            if cover == "ERR"{
                println!("{}Warning: unable to download cover for \"{}\"{}", YELLOW, title, CLR);
            }
        }
        else if todo[x].cover == "None" && todo[x].is_file_url{
            cover = download_cover_art(&infile, &title);
        }
        else{
            cover = (&todo[x].cover).to_string();
        }
        let outfile:String = ensure_string_terminates_with_fwd_slash(&outdir) + &(title.clone() + &fex).trim().to_owned();

        final_ffmpeg(&cover, &outfile, &infile);
        if todo[x].is_file_url{std::process::Command::new("rm").arg("-f").arg(infile).status().expect("Error");}
        if todo[x].is_cover_url && cover != "None" {std::process::Command::new("rm").arg("-f").arg(cover).status().expect("Error");}
        


        x+=1;
    }
    
}


fn download_cover_art(infile: &String, title: &String) -> String{
    if infile.contains("https://youtube.com"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtube.com/watch?v=").collect::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        println!("Failed to automatically download cover art for \"{}\". This can be ignoRED.", title);
        return "None".to_string();
    }
    if infile.contains("https://youtu.be"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtu.be/").collect::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        println!("Failed to automatically download cover art for \"{}\". This can be ignoRED.", title);
        return "None".to_string();
    }
    return "None".to_string();
}

fn final_ffmpeg(cover: &String, outputfile: &String, infile: &String){
    if cover == "None"{
        match std::process::Command::new("ffmpeg")
            .arg("-i").arg(infile.trim().to_owned())
            .arg(outputfile).status(){
                Ok(_) => println!("{}created file {}{}", GREEN, outputfile, CLR),
                Err(_) => println!("{}fatal ffmpeg error on file {}{}", RED, outputfile, CLR)
            }
    }
    else{ 
        match std::process::Command::new("ffmpeg")
            .arg("-i").arg(infile.trim().to_owned())
            .arg("-i").arg(cover.trim().to_owned())
            .arg("-map").arg("0").arg("-map").arg("1")
            .arg(outputfile).status(){ 
                Ok(_) => println!("{}created file {}{}", GREEN, outputfile, CLR),
                Err(_) => println!("{}fatal ffmpeg error on file {}{}", RED, outputfile, CLR)
            }
    }
}



fn wget_cover(url:&String)->String{
    let mut newfilename = gen_filename(&"".to_string());
    newfilename = match 
        std::process::Command::new("wget").arg(url).arg("-o").arg(newfilename.clone()).status()
    {
        Ok(_) => newfilename,
        Err(_) => "ERR".to_string()

    };
    return newfilename
}

fn gen_filename(fex: &String) -> String{
    let mut length = 100;
    let chars_allowed = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890-_".chars().collect::<Vec<char>>();
    let mut fname = "/tmp/".to_string();
    length-=fname.len() + fex.len();
    
    while length != 0{
        length-=1;
        fname.push(chars_allowed[Uniform::from(0..chars_allowed.len()-1).sample(&mut rand::thread_rng())]);
    }
    return fname.to_owned() + fex;

}

fn tmp_ytdlp(url: &String, fex: &String) -> String{
    let mut newfilename = gen_filename(&(".".to_owned() + fex));
    newfilename = match 
        std::process::Command::new("yt-dlp").arg(url).arg("-x").arg("--audio-format").arg("flac").arg("-o").arg(newfilename.clone()).status()
    {
        Ok(_) => newfilename,
        Err(_) => "ERR".to_string()

    };
    return newfilename

}

pub fn ensure_string_terminates_with_fwd_slash(string: &String) -> String{
    if string.chars().nth(string.len()-1).unwrap() != '/'{
        return string.to_owned()+"/";
    }
    return string.to_string();
}

fn is_done(title: &String, dir: &String, fex: &String) -> bool{
    let theoretical_file_name = ensure_string_terminates_with_fwd_slash(dir) + title + fex;
    let files = fs::read_dir(dir).unwrap();
    for file in files {
        if theoretical_file_name == file.unwrap().path().display().to_string(){
            return true;
        }
    }
    return false;
}
