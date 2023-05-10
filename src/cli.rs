use clap::{command, Parser};

/// Convert images to ASCII art
#[derive(Parser, Debug, Default)]
#[command(author, version, long_about = None)]
pub struct Args {
    /// Width of the output
    #[arg(long)]
    pub width: u32,

    /// Height of the output
    #[arg(long)]
    pub height: u32,

    /// The file to convert
    #[arg(short, long)]
    pub file: String,
}
