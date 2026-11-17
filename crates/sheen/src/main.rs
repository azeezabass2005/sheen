use clap::Parser;
use sheen::cli;
use std::fs;

fn main() {
    println!("This is sheen");
    let args = cli::Args::parse();
    println!("Arguments: {:?}", args);
    // Normally I would pass it to the markdown parser and mermaid parser
    let file_content = fs::read_to_string(args.filename);
    println!("This is the raw markdown file: \n\n {:?}", file_content);
}
