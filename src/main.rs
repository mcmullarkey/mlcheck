use clap::Parser;

use mlcheck::cli::{CliArgs, Config};

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let config = Config::from_args(args).map_err(|e| anyhow::anyhow!(e))?;
    mlcheck::run(config).map_err(|e| anyhow::anyhow!(e))?;
    Ok(())
}
