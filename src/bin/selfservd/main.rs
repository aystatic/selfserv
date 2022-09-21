use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, parse(from_occurrences))]
    verbose: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    Ok(())
}
