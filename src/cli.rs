use clap::{command, Parser};

/// Convert images to ASCII art
#[derive(Parser, Debug, Default)]
#[command(author, version, long_about = None)]
pub struct Args {
    /// The file to convert
    pub file: String,

    /// Width of the output
    #[arg(short = 'W', long)]
    pub width: u32,

    /// Height of the output
    #[arg(short = 'H', long)]
    pub height: u32,

    /// Output the result to a file
    #[arg(short, long)]
    pub output: Option<String>,

    // -- Optional commands --
    /// Invert the color of the output
    #[arg(short, long)]
    pub invert: bool,

    /// Omit stdout output. Only available when `--output` specified
    #[arg(short, long, requires("output"))]
    pub quiet: bool,
}
