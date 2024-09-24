use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'i', long = "input")]
    input: std::path::PathBuf,
    #[arg(short = 'o', long = "output")]
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    println!("Hello, world!");
    println!(
        "input path: {:?}, output path: {:?}",
        args.input, args.output
    )
}
