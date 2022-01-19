mod cli;
mod generator;
mod testspec;

fn main() -> anyhow::Result<()> {
    cli::execute()
}
