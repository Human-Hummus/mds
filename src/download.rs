use std::fs;
use rand;
use crate::parser::SongDesc;
use rand::distributions::Uniform;
use rand::distributions::Distribution;
mod cover;
use crate::*;

pub fn download(todo: Vec<SongDesc>, outdir: String, filetype: char){
    let mut x = 0;
    while x < todo.len(){
        let infile:String;
        let cover:String;
        let title = todo[x].name.clone();
        
        if is_done(&title, &outdir){alert!(format!("file \"{}\" is already present.", title));x+=1;continue;}

        cover = cover::process_cover(&todo[x].cover, todo[x].is_cover_url, todo[x].is_file_url, &todo[x].infile, &title);
        
        if todo[x].is_file_url{
            alert!(format!("tdx {:?}", todo[x]));
            infile = tmp_ytdlp(&todo[x].infile);
            if infile == "ERR"{
                error!(format!("Fatal error while downloading \"{}\".", title));
                x+=1;
                continue;
            }
        }
        else {infile = todo[x].infile.clone();}

        let final_fex = find_file_extension(filetype);

        let outfile:String = format!("{}{}", ensure_string_terminates_with_fwd_slash(&outdir), (title.clone() + &final_fex).trim().to_owned());

        final_ffmpeg(&cover, &outfile, &infile, filetype);
        if todo[x].is_file_url{std::process::Command::new("rm").arg("-f").arg(infile).status().expect("Error");}
        if todo[x].is_cover_url && cover != "None" {std::process::Command::new("rm").arg("-f").arg(cover).status().expect("Error");}
        x+=1;
    }
    
}

fn find_file_extension(ftype:char) -> String{
    return match ftype{
        'f' => ".flac",
        'o' => ".opus",
        'm' => ".mp3",
        'w' => ".wav",
        'v' => ".ogg",
        'a' => ".aac",
        _ => panic!("unknown filetype")
    }.to_string()
}

fn final_ffmpeg(cover: &String, outputfile: &String, infile: &String, ftype: char){
    let codec:String = match ftype{
        'f' => "flac",
        'o' => "libopus",
        'm' => "mp3",
        'w' => "wav",
        'v' => "libvorbis",
        'a' => "aac",
        _ => panic!("unknown filetype")
    }.to_string();
    let bitrate:String = match ftype{
        'f' => "0",
        'o' => "192k",
        'm' => "320k",
        'w' => "0",
        'v' => "224k",
        'a' => "256k",
        _ => panic!("unknown filetype")
            
    }.to_string();
    if cover == "None"{
        println!("1");
        match std::process::Command::new("ffmpeg")
            .arg("-i").arg(infile.trim().to_owned())
            .arg("-c:a").arg(codec)
            .arg("-b:a").arg(bitrate)
            .arg(outputfile).status(){
                Ok(_) => alert!(format!("created file {}", outputfile)),
                Err(_) => {error!(format!("fatal ffmpeg error on file {}", outputfile))}
            }
    }
    else{ 
        println!("2");
        match std::process::Command::new("ffmpeg")
            .arg("-i").arg(infile.trim().to_owned())
            .arg("-i").arg(cover.trim().to_owned())
            .arg("-map").arg("0:a").arg("-map").arg("1:v")
            .arg("-c:a").arg(codec)
            .arg("-b:a").arg(bitrate)
            .arg("-disposition:1").arg("attached_pic")
            .arg(outputfile).status(){ 
                Ok(_) => alert!(format!("created file {}",outputfile, )),
                Err(_) => error!(format!("fatal ffmpeg error on file {}",  outputfile))
            }
    }
}


pub fn gen_filename(fex: &String) -> String{
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

fn tmp_ytdlp(url: &String) -> String{
    let mut newfilename = gen_filename(&".flac".to_owned());
    newfilename = match 
        std::process::Command::new("yt-dlp")
        .arg(url).
        arg("-x").
        arg("--audio-format").arg("flac").
        arg("-o").arg(newfilename.clone()).status(){
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

fn is_done(title: &String, dir: &String) -> bool{
    let theoretical_file_name = ensure_string_terminates_with_fwd_slash(dir) + title;
    let files = fs::read_dir(dir).unwrap();
    for file in files {
        let file_no_ex = remove_fex(file.unwrap().path().display().to_string());
        if theoretical_file_name == file_no_ex{
            return true;
        }
    }
    return false;
}


//remove file extension
fn remove_fex(mut filename: String) -> String{
    while filename.len() > 1 && filename.pop().unwrap() != '.'{}
    return filename;
}
