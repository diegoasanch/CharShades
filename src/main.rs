use clap::Parser;

mod cli;
pub mod converter;

fn main() {
    let args = cli::Args::parse();

    // Generate ASCII representation
    let result = match converter::image_to_ascii(&args.file, args.width, args.height, args.invert) {
        Ok(ascii) => ascii,
        Err(e) => {
            if !args.quiet {
                    eprintln!("Error generating ascii: {}", e);
            }
            return 1;
        }
    };

    // Save result to file if specified
    if let Some(output) = args.output {
        match std::fs::write(&output, &result) {
            Ok(_) => {
                if !args.quiet {
                    println!("Result saved in {}", output);
                }
            }
            Err(e) => {
                eprintln!("Error saving result: {}", e);
                return 1;
            }
        }
    }

    // Print result to stdout if not quiet
    if !args.quiet {
        println!("{}", &result);
    }
}
