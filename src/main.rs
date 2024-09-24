use clap::Parser;
use image::ImageReader;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'i', long = "input")]
    input: std::path::PathBuf,
    #[arg(short = 'o', long = "output")]
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let img = ImageReader::open(&args.input).unwrap().decode().unwrap();

    println!("Hello, world!");
    println!(
        "input path: {:?}, output path: {:?}",
        args.input, args.output
    )
}
