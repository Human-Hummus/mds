use crate::*;

pub fn search_files(songs: Vec<SongDesc>, query: String) {
    let mut said_results = false;
    let mut x = 0;
    while x < songs.len() {
        if songs[x].name.to_lowercase().contains(&query) {
            if !said_results {
                said_results = true;
                info!("Results:\n\n");
            }
            info!(
                "\t\"{}\":\n\t\t{}\n\t\t{}:\"{}\"\n\t\t{}\n",
                songs[x].name,
                match songs[x].artist.as_str() {
                    "" => "No artist provided".to_string(),
                    _ => format!("Artist: {}", songs[x].artist),
                },
                match songs[x].is_file_url {
                    true => "Source URL: ",
                    false => "Source File: ",
                },
                songs[x].infile,
                match &songs[x].cover {
                    None => "No Cover Provided".to_string(),
                    Some(teh_cover) => format!(
                        "{}\"{}\"",
                        match songs[x].is_cover_url {
                            true => "Provided Cover URL: ",
                            false => "Provided Cover File: ",
                        },
                        teh_cover
                    ),
                }
            );
        }
        x += 1;
    }
    if !said_results {
        info!("No results.")
    }
}
