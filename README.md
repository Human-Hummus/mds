# MDS - Music Download Script.
This is what it says on the tin; it downloads music. It can copy music from local files, too. It's built using ffmpeg & yt-dlp.

 The reason I made this is because while yt-dlp can download a list of songs, and I can easily copy local files, yt-dlp can't know if a song is already downloaded or not until it sees the title. Additionally, it's hard to make yt-dlp output multiple files with custom titles. Also, cover art isn't included with yt-dlp's outputted files(usually), and it'd be nice to combine compressing local files and downloading remote files into one script. This is, by the way, a script and not much more. It's written in Rust because of course it is. I had written an earlier version of this in Python, but I've rewritten it in Rust because, well, it's fun. That and it's cooler. Also, (in my opinion) Python is a poor choice for this project. 


The format of the input file(s) is as follows:

<mark>\#this is a comment</mark>

The comment starts with a hash(\#) symbol.

<mark>\*/home/username</mark>

This one's less obvious. This is the variable that is appended to the beginning of input music files, so "yeet.mp3" will now become "/home/username/yeet.mp3". This should always be a directory. The default value is "/".

<mark>yeet.mp3 | yeetus | cover.png</mark>

 This is even less obvious. there are three values seperated by pipe(|) symbols. Note that the third option(cover.png) can be ommited, and the line changed to "<mark>yeet.mp3 | yeetus</mark>". Another thing to note is that whitespace preceeding and proceeding each value will also be ommited. The first value(yeet.mp3) is the input file, this stream will be copied. The second value(yeetus) is the title of the song. The outputted file for this line will be yeetus.(mp3/opus/flac), based on the title.  The third and final value(cover.png) is the cover art. Note that the cover art file also has the default or specified directory appended to the beginning of it. 


 Another thing that is **very** important: the excamation mark(!). when the ! is appended to the beginning of the value for the cover art or input file, i.e(!yeet.mp3 | ... | !cover.png), the file will be treated as a link to a remote file. This link will be passed to yt-dlp in the case of the input file, and wget in the case of the cover art. It will then be  converted into a local file in the /tmp/ directory, and the process will continue from there as normal. 

 Also, it's possible, if you supply a link and not a cover, that MDS will be able to find the cover art on its own. This is rather buggy, and currently only works with Soundcloud and Youtube, but it's pretty neat. 
