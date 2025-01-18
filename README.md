
# MDS - Music Download Script


MDS does what it says on the tin: it downloads music! (or copies them from local files!) It's built using ffmpeg, yt-dlp, wget, and, of course, Rust! I made MDS because, while yt-dlp *can* download a list (like a YouTube playlist) of songs, it doesn't know if a song is already downloaded until it sees the title on YouTube or whatever, and thus, every time you want to update your local folder, it must check every single page, which is slow and can get real spammy, if you've got a large playlist. Additionally, it's hard to make yt-dlp output multiple files with custom titles. Cover art isn't included with yt-dlp's outputted files, either, and you can't set your own cover if you don't like the pre-determined one. And what about local media? It'd be nice to manage **ALL** songs with one file. This program can download many files at once, with custom covers and titles, and supports local media sources.


## Input File Content

### Songs
Adding a song to an input file is as follows:
$source | ($artist\_name -)  $song\_name  | ($cover)
Everything in parenthesis is optional
Variable|Meaning|
|-|-|
$source|The source file, relative to the directory containing the input file or URL. If it's a URL, it must be preceeded by an exclaimation mark (!).|
$artist\_name|The name of the artist of the song. This'll also be included in the filename AND embedded in the file. This is an optional component|
$song\_name|The name of the song, and the name of the output file (aside from the file's extension). Required.|
$cover|Cover image of the song. The format of this is identical to source with regaurds to the URL stuff. Optional.|



Note that MDS will attempt to automatically get the cover of a YouTube or SoundCloud song if one isn't provided by the user.


Example:
!https://www.youtube.com/watch?v=dQw4w9WgXcQ | Rick Astley - Never Gonna Give You Up | !https://www.blender.org/wp-content/themes/bthree/assets/icons/favicon-32x32.png


This sets Never Gonna Give You Up from YouTube as the source, sets Rick Astley as the artist, and uses the Blender.org favicon as the cover, which it downloads. The outputted file would be named "Rick Astley - Never Gonna Give You Up.ogg".


Example \#2:
song.mp4 | artist - song | cover.png


This would get the audio of song.mp4, set the song name to artistsong, and the cover image to cover.png. The outputted file would be named "artist - song.ogg". Also, song.mp4 and cover.png would need to be in the same directory. Though, you could make it ../cover.png or whatever, if you want. Unless...


### Changing the Input Directory
By default, song sources and covers will be taken relative to the directory of the current input file. You can set it to another directory by starting a line with the asterisk (\*) and then writing the new directory you want to use.
For example:\*/home/user
Would set the default directory to /home/username (if the path doesn't begin with a slash, it will ALSO be relative to the current file). This applies only to the current file.
### Imports
You can import another input file by starting a line with an at sign (@) and writing the path to the other file, relative to the current directory MDS is using.
For example:
@poop/input.txt
This'd also get the songs from poop/input.txt
### Comments
\#This is a comment
Simple enough; Comments start with a hash(\#) symbol and continue to the end of the line. Comments must start at the beginning of a line, though it may be preceeded by whitespace.
## Installing/Updating\*
\*Linux exclusive. \*BSD and MacOS would probably work, but it's untested.
You must have the following software installed:
- [yt-dlp](https://github.com/yt-dlp/yt-dlp)
- [FFmpeg](https://ffmpeg.org/)
- [Wget](https://www.gnu.org/software/wget/)
- [Cargo (Rust)](https://crates.io/)


### Commands

- git clone [this repository](https://github.com/Human-Hummus/mds)
- cd into the directory
- run sudo make install
- MDS is now installed and can be invoked with "mds"

Note that MDS is installed to /usr/bin. This can cause issues on some (typically fairly exotic) Linux distibutions, namely NixOS.
## Command Line Flags

Flag|Abbreviation|Purpose|
|-|-|-|
--input [file]|-i [file]|Set the input file to "[file]"|
--output [dir]|-o [dir]|Set output directory to "[dir]"|
--format [format]|-f [format]|Set output files to the format "[format]". Note that this format can be different to that of other files in the output directory; it'll be fine.|
--search [query]|-s [query]|Search through song titles for the string "[query]"|
--help|-h|Print help info|
--delete-not-present|N/A|Delete files in the output directory that aren't described in the input file(s). This will print several warnings before deleting any files.|
--do-not-warn|-n|Omit the warnings described above|
--quiet|-q|Suppress logs consisting of anything other than errors or warnings.|
--silent|-Q|Suppress logs consisting of anything other than errors.|
--verbose|-v|Don't suppress output (default behavior)|

