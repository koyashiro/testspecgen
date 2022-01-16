mod cli;
mod testspec;

use anyhow::Result;

fn main() -> Result<()> {
    cli::execute()
}
