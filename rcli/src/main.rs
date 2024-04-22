use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();

    match opts.cmd {
        Subcommand::CSV(opts) => {
            process_csv(&opts.input, &opts.output)?
        }
    }

    Ok(())
}
