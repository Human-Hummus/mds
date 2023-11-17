use crate::*;
use std::{fs::read_to_string, path::Path};

#[derive(Debug)]
pub struct SongDesc {
    pub name:           String,
    pub infile:         String,
    pub is_file_url:    bool,
    pub cover:          String,
    pub is_cover_url:   bool,
}
impl SongDesc{
    pub fn clone(&self) -> SongDesc{
        return SongDesc{
            name:           self.name.clone(),
            infile:         self.infile.clone(),
            is_file_url:    self.is_file_url,
            cover:          self.cover.clone(),
            is_cover_url:   self.is_cover_url
        }
    }
}
fn lines_to(chars: &Vec<char>, pos: usize) -> usize{
    if pos >= chars.len(){return usize::MAX}
    let [mut x, mut lines] = [0,0];
    while x < pos{
        if chars[x] == '\n'{
            lines+=1;
        }
        x+=1;
    }
    lines
}


pub fn parse_file(path:&String) -> Vec<SongDesc>{
    let text = read_to_string(path).unwrap_or_else(|_| fatal!(format!("Fatal Error: unable to read file \"{}\"", path)))
        .chars().collect::<Vec<char>>();

    let mut toret:Vec<SongDesc> = Vec::new();
    let mut x = 0;
    let mut current_home = Path::new(path).parent().unwrap().to_string_lossy().into_owned() + "/";
    while x < text.len(){
        match text[x] {
            '\n' => {x+=1},
            '#' => {
                while x < text.len() && text[x] != '\n'{x+=1}
            },
            '/' | 'a'..='z' | 'A'..='Z' | '1'..='9' | '-' | '.' | ',' | '!' => {
                let mut song = SongDesc{
                    name:           String::new(),
                    infile:         String::new(),
                    is_file_url:    false,
                    cover:          String::new(),
                    is_cover_url:   false,
                };

                if text[x] == '!'{song.is_file_url = true;x+=1}

                while text[x] != '|'{
                    song.infile.push(text[x]);
                    x+=1;
                    if x == text.len() || text[x] == '\n'{fatal!(format!("In file \"{}\" on line \"{}\" there is an incomplete or invalid command", path, lines_to(&text, x)))}
                }
                x+=1;
                song.infile = song.infile.trim().to_owned();
                
                if !song.is_file_url {song.infile = current_home.clone() + &song.infile}
                if song.infile.len() < 1 {fatal!(format!("In file \"{}\" on line \"{}\" there is an incomplete or invalid command", path, lines_to(&text, x)))}

                while x < text.len(){
                    song.name.push(text[x]);
                    x+=1;
                    if text[x] == '\n'{break}
                    if text[x] == '|'{x+=1;break}
                }

                song.name = song.name.trim().to_owned();
                if song.name.len() < 1 {fatal!(format!("In file \"{}\" on line \"{}\" there is an incomplete or invalid command", path, lines_to(&text, x)))}

                while x < text.len() && text[x] != '\n'{
                    song.cover.push(text[x]);
                    x+=1
                }

                song.cover = song.cover.trim().to_owned();
                
                if song.cover.len() > 0{
                    if song.cover.chars().nth(0).unwrap() == '!'{
                        song.is_cover_url = true;
                        song.cover = song.cover.chars().collect::<Vec<char>>()[1..song.cover.len()-1].iter().cloned().collect::<String>()
                    }
                    else {
                        song.cover = format!("{}{}", current_home, song.cover)
                    }
                }
                else{song.cover = String::from("None")}
                toret.push(song);
            },
            '*' => {
                x+=1;
                current_home = String::new();
                while x < text.len() && text[x] != '\n'{
                    current_home.push(text[x]);
                    x+=1;
                }
                let mut current_home_tmp = current_home.trim().chars().collect::<Vec<char>>();
                if current_home_tmp[current_home_tmp.len()-1] != '/'{
                    current_home_tmp.push('/');
                }
                current_home = current_home_tmp.into_iter().collect();

            },
            '@' => {
                x+=1;
                let mut file_to_go = current_home.clone();
                let mut ftga = String::new();
                while x < text.len() && text[x] != '\n'{
                    ftga.push(text[x]);
                    x+=1;
                }
                ftga = ftga.trim().to_owned();
                if ftga.chars().nth(0).unwrap() == '/'{
                    ftga = ftga.chars().collect::<Vec<char>>()[1..ftga.len()-1].into_iter().collect()
                }
                file_to_go += &ftga;
                toret.append(&mut parse_file(&file_to_go));
            },
            _ => {x+=1}

        };
    }
    toret
}
