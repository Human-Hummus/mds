use crate::download::gen_filename;
use crate::*;

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
    if infile.contains("https://youtube.com") || infile.contains("https://www.youtube.com"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtube.com/watch?v
=").collect::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        debug!(format!("Failed to automatically download cover art for \"{}\". This can be ignored.", title));
        return "None".to_string();
    }
    if infile.contains("https://youtu.be"){
        let toret = wget_cover(&("https://i.ytimg.com/vi/".to_owned() + &(infile.split("https://www.youtu.be/").collect
::<Vec<&str>>()[1].to_owned() + "/hqdefault.jpg")));
        if !(toret == "ERR"){
            return toret;
        }
        debug!(format!("Failed to automatically download cover art for \"{}\". This can be ignored.", title));
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

