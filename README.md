# MDS - Music Download Script.

MDS does what it says on the tin: it downloads music! (Or copies them from local files!) It's built using ffmpeg, yt-dlp, wget, and, of course, Rust!

I made MDS because, while yt-dlp *can* download a list (like a YouTube playlist) of songs, it doesn't know if a song is already downloaded until it sees the title on YouTube or whatever, and thus, for every time you want to update your local folder, it's gotta check every single page, which is slow and can get real spammy, if you've got a large playlist. 
Additionally, it's hard to make yt-dlp output multiple files with custom titles. Cover art isn't included with yt-dlp's outputted files, either, and you can't set your own cover if you don't like the pre-determined one. 
And what about local media? It'd be nice to manage **ALL** songs with one file.

This program can download many files at once, with custom covers and titles, and supports local media sources.


##Input file content:

###Songs
Adding a song to an input file is as follows:
$source | ($artist\_name - ) $song\_name ( | $cover)

Everything in parenthesis is optional

| Variable | Meaning
|---            |---|
|$source        | The source file, relative to the directory containing the input file, OR URL. If it's a URL it must be preceeded by an exclaimation mark (!). Required.
|$artist\_name  | The name of the artist for the song. This will also be included in the name of the song, and embeded in the file. Optional.
|$song\_name    | The name of the song, and the name of the output file (aside from the file extension). Required.
|$cover         | The cover image of the song. The format is identical to $source. Optional.

MDS will attempt to automatically get the cover of a YouTube or SoundCloud, if one isn't provided.

Example:
!https://www.youtube.com/watch?v=dQw4w9WgXcQ | Rick Astley - Never Gonna Give You Up | !https://www.blender.org/wp-content/themes/bthree/assets/icons/favicon-32x32.png

This sets Never Gonna Give You Up from YouTube as the source, sets Rick Astley as the artist,
and uses the Blender.org favicon as the cover, which it downloads. The outputted file would be named "Rick Astley - Never Gonna Give You Up.ogg".

song.mp4 | artist - song | cover.png

This would get the audio of song.mp4, set the song name to "artist - song", and the cover image to cover.png. The outputted file would be named "artist - song.ogg".
Also, song.mp4 and cover.png would need to be in the same directory. Though you could make it ../cover.png or whatever if you want. Unless...



###Changing input directory.

By default, song sources and covers will be taken relative to the directory of the current input file.
You can set it to another directory by starting a line with the asterisk (\*) and then writing the new directory you want to use.

For example:
\*/home/username

Would set the default directory to /home/username (if the path doesn't begin with a slash, it will ALSO be relative to the current file).
This applies **only** to the current file.


###Imports
You can import another input file by starting a line with an at sign (@) and writing the path to the other file, relative to the current directory MDS is using.
For example:
@poop/input.txt

Would also get the songs from poop/input.txt



###Comments
\#this is a comment
Comments starts with a hash(\#) symbol and continue to the end of the line.
Comments must start at the **begining** of a line (whitespace is ignored).


##Flags
|Flag | Short | purpose|
|---|---|---|
| --input [file]| -i [file]| Set the input file to"[file]"|
| --output [dir]| -o [dir]| Set the output directory to "[dir]"|
| --format [format] | -f [format]| Set the output files to the format "[format]"|
| --search [query] | -s [query] | Search through song titles for "[query]".
| --help | -h | Print help info|
| --delete-not-present || Delete files in the directory that aren't in the input file, this will print several warnings and ask before deleting any files.|
| --do-not-warn | -n | Don't print the warnings or ask before deleting files.|
| --quiet | -q | Supress logs consisting of anything other than errors or warnings.|
| --silent| -Q | Supress logs consisting of anything other than errors.|
| --verbose| -v| Don't supress output(default).|
