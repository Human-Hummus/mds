use std::env;
mod download;
mod parser;



fn safe_read_f(filepath: &String) -> String{
    return match std::fs::read_to_string(filepath){
        Ok(file) => {file},
        Err(_) => {eprintln!("fatal error: unknown file \"{}\"", filepath); std::process::exit(1);}
    }
}
fn safe_read_d(dirpath: &String) -> Vec<String>{
    return match std::fs::read_dir(dirpath){
        Ok(file) => {
                let mut out:Vec<String> = [].to_vec();
                for i in file{
                    out.push(match i{
                        Ok(path) => {path.path()}
                        Err(_) => {eprintln!("fatal error: error reading directory \"{}\"", dirpath);std::process::exit(1);}
                    }.display().to_string());
                }
                return out;
            },
        Err(_) => {eprintln!("fatal error: unknown file \"{}\"", dirpath); std::process::exit(1);}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut x = 1;
    let fex = ".flac";
    let mut text:String = String::new();
    let mut output = String::new();

    while x < args.len(){
        if args[x] == "--help" || args[x] == "-h"{
            println!("{}", include_str!("help.txt"));
            std::process::exit(0);
        }
        else if args[x] == "-i" || args[x] == "--input"{
            x+=1;

            //check that the argument is complete.
            if x == args.len(){
                eprintln!("fatal error: incomplete input argument.");
                std::process::exit(1);
            }

            //read the file.
            text+=&(safe_read_f(&args[x]) + "\n");
            x+=1;
        }
        else if args[x] == "-o" || args[x] == "--output"{
            x+=1;
            //check for a complete argument
            if x == args.len(){eprintln!("fatal error: output directory arguemnt incomplete.");std::process::exit(1);}

            safe_read_d(&args[x]);
            output = args[x].to_string();
            x+=1;
        }
        else{
            println!("warning: unknown argument: \"{}\"", args[x]);
            x+=1;
        }
    }
    let stuff = parser::parse(&text);
    download::download(stuff, output, fex.to_string());


}
