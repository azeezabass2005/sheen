use crate::markdown_path::MarkdownPath;
use clap::Parser;

/// CLI arguments structs
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the file you want to render
    pub filename: MarkdownPath,
}
