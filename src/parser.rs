use std::{fs::read_to_string, path::Path};
extern crate log;


#[derive(Debug, Clone)]
pub struct SongDesc {
    pub artist: String,
    pub name: String,
    pub infile: String,
    pub is_file_url: bool,
    pub cover: Option<String>,
    pub is_cover_url: bool,
}
impl SongDesc {
    pub fn clone(&self) -> SongDesc {
        return SongDesc {
            artist: self.artist.clone(),
            name: self.name.clone(),
            infile: self.infile.clone(),
            is_file_url: self.is_file_url,
            cover: self.cover.clone(),
            is_cover_url: self.is_cover_url,
        };
    }
}
fn lines_to(chars: &Vec<char>, pos: usize) -> usize {
    if pos >= chars.len() {
        return usize::MAX;
    }
    let [mut x, mut lines] = [0, 0];
    while x < pos {
        if chars[x] == '\n' {
            lines += 1;
        }
        x += 1;
    }
    lines
}

pub fn parse_file(path: &String) -> Vec<SongDesc> {
    let text = read_to_string(path)
        .unwrap_or_else(|_| panic!("Fatal Error: unable to read file \"{}\"", path))
        .chars()
        .collect::<Vec<char>>();

    let mut toret: Vec<SongDesc> = Vec::new();
    let mut x = 0;
    let mut current_home = Path::new(path)
        .parent()
        .unwrap()
        .to_string_lossy()
        .into_owned()
        + "/";
    while x < text.len() {
        match text[x] {
            '\n' => x += 1,
            '#' => {
                while x < text.len() && text[x] != '\n' {
                    x += 1
                }
            }
            '/' | 'a'..='z' | 'A'..='Z' | '1'..='9' | '-' | '.' | ',' | '!' => {
                let mut song = SongDesc {
                    artist: String::new(),
                    name: String::new(),
                    infile: String::new(),
                    is_file_url: false,
                    cover: None,
                    is_cover_url: false,
                };

                if text[x] == '!' {
                    song.is_file_url = true;
                    x += 1
                }

                while text[x] != '|' {
                    song.infile.push(text[x]);
                    x += 1;
                    if x == text.len() || text[x] == '\n' {
                        panic!("In file \"{}\" on line \"{}\" there is an incomplete or invalid command", path, lines_to(&text, x))
                    }
                }
                x += 1;
                song.infile = song.infile.trim().to_owned();

                if !song.is_file_url {
                    song.infile = current_home.clone() + &song.infile
                }
                if song.infile.len() < 1 {
                    panic!(
                        "In file \"{}\" on line \"{}\" there is an incomplete or invalid command",
                        path,
                        lines_to(&text, x)
                    )
                }

                while x < text.len() {
                    song.name.push(text[x]);
                    x += 1;
                    if text[x] == '\n' {
                        break;
                    }
                    if text[x] == '|' {
                        x += 1;
                        break;
                    }
                }

                song.name = song.name.trim().to_owned();
                if song.name.len() < 1 {
                    panic!(
                        "In file \"{}\" on line \"{}\" there is an incomplete or invalid command",
                        path,
                        lines_to(&text, x)
                    )
                }
                let mut cover = String::new(); 
                while x < text.len() && text[x] != '\n' {
                    cover.push(text[x]);
                    x += 1
                }

                cover = cover.trim().to_owned();

                if cover.len() > 0 {
                    if cover.chars().nth(0).unwrap() == '!' {
                        song.is_cover_url = true;
                        song.cover = Some(cover.chars().collect::<Vec<char>>()
                            [1..cover.len()]
                            .iter()
                            .cloned()
                            .collect::<String>())
                    } else {
                        song.cover = Some(format!("{}{}", current_home, cover))
                    }
                } else {
                    song.cover = None
                }
                toret.push(song);
            }
            '*' => {
                x += 1;
                current_home = String::new();
                while x < text.len() && text[x] != '\n' {
                    current_home.push(text[x]);
                    x += 1;
                }
                let mut current_home_tmp = current_home.trim().chars().collect::<Vec<char>>();
                if current_home_tmp[current_home_tmp.len() - 1] != '/' {
                    current_home_tmp.push('/');
                }
                current_home = current_home_tmp.into_iter().collect();
            }
            '@' => {
                x += 1;
                let mut file_to_go = current_home.clone();
                let mut ftga = String::new();
                while x < text.len() && text[x] != '\n' {
                    ftga.push(text[x]);
                    x += 1;
                }
                ftga = ftga.trim().to_owned();
                if ftga.chars().nth(0).unwrap() == '/' {
                    ftga = ftga.chars().collect::<Vec<char>>()[1..ftga.len() - 1]
                        .into_iter()
                        .collect()
                }
                file_to_go += &ftga;
                toret.append(&mut parse_file(&file_to_go));
            }
            _ => x += 1,
        };
    }
    toret
}

pub fn get_artist_name(input:&mut Vec<SongDesc>){
    let mut x = 0;
    while x < input.len(){
        if !input[x].name.contains("-"){x+=1;continue}
        let mut y = 0;
        let title_chars:Vec<char> = input[x].name.chars().collect();
        let mut artist = String::new();
        while y < title_chars.len() && title_chars[y] != '-'{
            artist.push(title_chars[y]);
            y+=1;
        }
        input[x].artist=artist.trim().to_owned();
        x+=1;
    }
}
