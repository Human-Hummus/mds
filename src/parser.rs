use crate::download::ensure_string_terminates_with_fwd_slash;
use crate::*;

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
    let mut x = 0;
    let mut lines = 0;
    while x < pos{
        if chars[x] == '\n'{
            lines+=1;
        }
        x+=1;
    }
    return lines;
}

pub fn parse_file(path:&String) -> Vec<SongDesc>{
    let text = match std::fs::read_to_string(path) {
        Ok(data) => data,
        Err(_) => fatal!(format!("Fatal Error: unable to read file \"{}\"", path))
    }.chars().collect::<Vec<char>>();

    let mut toret:Vec<SongDesc> = Vec::new();
    let mut x = 0;
    let mut current_home = get_containing_dir(path.clone());
    while x < text.len(){
        match text[x] {
            '#' => {
                while x < text.len() && text[x] != '\n'{
                    x+=1;
                }
            },
            '*' => {
                current_home.clear();
                x+=1;
                while x < text.len() && text[x] != '\n'{
                    current_home.push(text[x]);
                    x+=1;
                }
                current_home = ensure_string_terminates_with_fwd_slash(current_home.trim());
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

                while x < text.len() && text[x] != '|'{
                    song.infile.push(text[x]);
                    x+=1;
                    if x == text.len() || text[x] == '\n'{fatal!(format!("In file \"{}\" on line \"{}\" there is an incomplete or invalid command", path, lines_to(&text, x)))}
                }
                x+=1;
                song.infile = song.infile.trim().to_owned();
                if song.is_file_url == false{song.infile = current_home.clone() + &song.infile.clone()}
                if song.infile.len() < 1 {fatal!(format!("In file \"{}\" on line \"{}\" there is an incomplete or invalid command", path, lines_to(&text, x)))}
                while x < text.len() && text[x] != '|' && text[x] != '\n'{
                    song.name.push(text[x]);
                    x+=1;
                }if text[x] == '|'{x+=1}
                song.name = song.name.trim().to_owned();
                if song.name.len() < 1 {fatal!(format!("In file \"{}\" on line \"{}\" there is an incomplete or invalid command", path, lines_to(&text, x)))}

                while x < text.len() && text[x] != '\n'{
                    song.cover.push(text[x]);
                    x+=1;
                }
                song.cover = song.cover.trim().to_owned();
                if song.cover.len() > 0{
                    let scc = song.cover.chars().collect::<Vec<char>>();
                    if scc[0] == '!'{
                        song.is_cover_url = true;
                        song.cover = scc[1..scc.len()-1].iter().cloned().collect::<String>();
                    }
                    else {
                        song.cover = format!("{}{}", current_home, song.cover)
                    }
                }
                else{song.cover = String::from("None")}
                toret.push(song);
            },
            '@' => {
                x+=1;
                let mut file_to_go = current_home.clone();
                let mut ftga = String::new();
                while x < text.len() && text[x] != '\n'{
                    ftga.push(text[x]);
                    x+=1;
                }
                file_to_go += &ensure_that_a_string_does_not_begin_with_a_forward_slash(ftga.trim());
                toret.append(&mut parse_file(&file_to_go));
            },
            _ => {x+=1}


        };
    }
    toret

}


#[inline(always)]
pub fn get_containing_dir(filepath: String) -> String{
    let mut chrs = filepath.chars().collect::<Vec<char>>();
    while chrs[chrs.len()-1] != '/'{chrs.pop();}
    let mut out = String::new();
    for ch in chrs{out.push(ch)}
    out
}


#[inline(always)]
fn ensure_that_a_string_does_not_begin_with_a_forward_slash(the_string_to_be_modified: &str) -> String{
    if the_string_to_be_modified.len() < 1{
        return String::from("");
    }
    if the_string_to_be_modified.chars().nth(0).unwrap() == '/'{
        return the_string_to_be_modified[1..the_string_to_be_modified.len()-1].to_owned();
    }
    return the_string_to_be_modified.to_string();
}
