use clap::Parser;

mod cli;
pub mod converter;

fn main() {
    let args = cli::Args::parse();

    match converter::image_to_ascii(&args.file, args.width, args.height) {
        Ok(ascii) => println!("{}", ascii),
        Err(e) => println!("Error generating ascii: {}", e),
    }
}
