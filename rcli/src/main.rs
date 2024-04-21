use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="rcli", version="0.1.0", author="Rust CLI", about, long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name="csv", about="Convert CSV to JSON")]
    CSV(CSVOpts)
}

#[derive(Debug, Parser)]
struct CSVOpts {
    #[arg(short, long, about="Input file path")]
    input:String,

    #[arg(short, long, about="Output file path")]
    
}

fn main() {
    println!("Hello, rust!");
}
