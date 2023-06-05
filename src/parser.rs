use crate::download::ensure_string_terminates_with_fwd_slash;
use crate::*;

#[derive(Debug)]
pub struct SongDesc {
    pub name: String,
    pub infile: String,
    pub is_file_url: bool,
    pub cover: String,
    pub is_cover_url: bool,
}

pub fn parse(text: &String) -> Vec<SongDesc>{
    let mut output:Vec<SongDesc> = Vec::new();
    let mut lines:Vec<Vec<char>> = vec![];
    for i in text.split("\n").collect::<Vec<&str>>(){
        lines.push(i.chars().collect::<Vec<char>>())
    };

    //the path appended to the start of a file path; this is what's defined by the *
    let mut default_path:String = String::from("/");

    //the line num variable is for more helpful error messages; this doesn't impact logic
    let mut line_num = 0;

    //floop means full loop.
    'floop: for line in lines{line_num+=1; let mut x = 0;if line.len() < 1{continue;}

        'linetype: while x < line.len(){
            if (!(x<line.len())) || (line[x] == '#'){
                continue 'floop;
            }
            match line[x]{
                '*' =>{
                        x+=1;
                        let mut new_default_path = String::new();
                        while x < line.len(){
                            new_default_path.push(line[x]);
                            x+=1;
                        }
                        default_path = ensure_string_terminates_with_fwd_slash(&new_default_path.trim().to_owned());
                        continue 'floop;
                    }
                '\n'|'\r'|' '|'\t' => {x+=1}
                _ =>{break 'linetype}
            }
        }
        if !(x<line.len()){
            continue;
        }

        let mut infile:String = String::new();
        let mut is_file_url = false;
        let mut title:String = String::new();
        let mut cover:String = String::new();
        let mut is_cover_url = false;
        
        if line[x] == '!'{x+=1;is_file_url=true;}

        while x < line.len() && line[x] != '|'{
            infile.push(line[x]);
            x+=1;
        }
        let _ = infile.trim().to_owned();
        if infile.len() == 0 {error!(format!("Error on line{}; unspecified input file.", line_num)); continue 'floop}
        x+=1;
        while x < line.len() && line[x] != '|'{
            title.push(line[x]);
            x+=1;
        }
        title = title.trim().to_owned();
        if title.len() == 0{error!(format!("Error on line {}; unspecified track name", line_num)); continue 'floop;}
        x+=1;
        while x < line.len() && line[x] != '|'{
            cover.push(line[x]);
            x+=1;
        }
        cover = cover.trim().to_owned();
        if cover.len() < 2{
            cover = String::from("None");
        }
        else{
            if cover.chars().nth(0).unwrap() == '!'{
                cover.remove(0);
                is_cover_url = true;
            }
        }

        output.push(
                SongDesc{
                    name: title.trim().to_owned(),
                    infile: (match is_file_url {
                                true => "".to_owned(),
                                false => default_path.clone()
                            } + &match is_file_url {
                                true => infile,
                                false =>  ensure_that_a_string_does_not_begin_with_a_forward_slash(&infile)
                            }).trim().to_owned(),
                    is_file_url: is_file_url,
                    cover: match is_cover_url {
                                true => "".to_owned(),
                                false => match cover == "None" {
                                    false => default_path.clone(),
                                    true => "".to_string()
                                }
                            } + &match is_cover_url {
                                true => cover,
                                false => ensure_that_a_string_does_not_begin_with_a_forward_slash(&cover),
                            },
                    is_cover_url: is_cover_url
                }
            );
    }
    return output;
}




fn ensure_that_a_string_does_not_begin_with_a_forward_slash(the_string_to_be_modified: &String) -> String{
    if the_string_to_be_modified.len() < 1{
        return String::from("");
    }
    if the_string_to_be_modified.chars().nth(0).unwrap() == '/'{
        return the_string_to_be_modified[1..the_string_to_be_modified.len()-1].to_owned();
    }
    return the_string_to_be_modified.to_string();
}
