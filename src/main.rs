mod cli;
mod generator;
mod testspec;

use anyhow::Result;

fn main() -> Result<()> {
    cli::execute()
}
