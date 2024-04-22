use clap::Parser;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name="rcli", version="0.1.0", author="Rust CLI", about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    CSV(CSVOpts),
}

#[derive(Debug, Parser)]
pub struct CSVOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        return Err("File not found".into());
    }
}
