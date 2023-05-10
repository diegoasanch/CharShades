use clap::Parser;

mod cli;
pub mod converter;

fn main() {
    let args = cli::Args::parse();

    let result = match converter::image_to_ascii(&args.file, args.width, args.height, args.invert) {
        Ok(ascii) => ascii,
        Err(e) => {
            eprintln!("Error generating ascii: {}", e);
            return;
        }
    };

    if let Some(output) = args.output {
        match std::fs::write(&output, &result) {
            Ok(_) => println!("Result saved in {}", output),
            Err(e) => eprintln!("Error saving result: {}", e),
        }
    }

    if !args.quiet {
        println!("{}", &result);
    }
}
