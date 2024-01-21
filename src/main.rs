use anyhow::{anyhow, Context, Result};
use clap_verbosity_flag::Verbosity;
use log::{debug, trace};
use std::{fs::File, io::{stdout, BufReader}};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[command(flatten)]
    verbose: Verbosity,
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn validate(args: Cli) -> Result<Cli> {
    if args.pattern.is_empty() {
        return Err(anyhow!("a value is required for '<PATTERN>' but none was supplied"))
    }
    Ok(args)
}

fn main() -> Result<()> {
    let args = validate(Cli::parse())?;

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();
    trace!("Starting up...");

    debug!("args: {:?}", &args);

    let f = File::open(&args.path)
        .with_context(|| format!("could not read file {}", args.path.display()))?;
    let reader = BufReader::new(f);

    grrs_gn::find_matches(reader, &args.pattern, &mut stdout())
}
