use clap::Parser;

mod file_list;
mod make_dir;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    println!("Welcome to rust core utils");
    //let args: Vec<String> = std::env::args().skip(1).collect();
    //let function = args[0];

    let function: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    match &function[..] {
        "ls" => file_list::ls(),
        "mkdir" => make_dir::mkdir(),
        _ => println!("Function not recognized")
    }
}

