extern crate termion;
use crate::parser::SongDesc;
use termion::color::Fg;
use termion::{color};
use crate::download::gen_filename;

const GREEN: Fg<color::Green> = color::Fg(color::Green);
const YELLOW: Fg<color::Yellow> = color::Fg(color::Yellow);
const RED: Fg<color::Red> = color::Fg(color::Red);
const CLR: Fg<color::Reset> = color::Fg(color::Reset);
const BLUE: Fg<color::Blue> = color::Fg(color::Blue);


//the output is a filename OR "None".
pub fn process_cover(og_cover: &String, is_url: bool, is_infile_link: bool, infile: &String, title: &String) -> String{
    if is_url{
        let new_cover = wget_cover(og_cover);
        if new_cover == "ERR"{
            println!("{}Alert: unable to download cover for \"{}\"{}", YELLOW, title, CLR);
            return String::from("None");
        }
        println!("{}Successfully downloaded cover art for \"{}\" automatically!{}",GREEN,title,CLR);
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
    if infile.contains("https://youtube.com") || infile.contains("https://www.youtube.com"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtube.com/watch?v
=").collect::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        println!("{}Failed to automatically download cover art for \"{}\". This can be ignored.{}", YELLOW, title, CLR);
        return "None".to_string();
    }
    if infile.contains("https://youtu.be"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtu.be/").collect
::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        println!("Failed to automatically download cover art for \"{}\". This can be ignored.", title);
        return "None".to_string();
    }
    return "None".to_string();
}

fn wget_cover(url:&String)->String{
    let mut newfilename = gen_filename(&"".to_string());
    newfilename = match 
        std::process::Command::new("wget").arg(url).arg("-O").arg(newfilename.clone()).status()
    {
        Ok(k) => match k.success(){
            true => newfilename,
            false => "ERR".to_string()
        },
        Err(_) => "ERR".to_string()

    };
    return newfilename
}

