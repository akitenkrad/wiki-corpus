use clap::{crate_version, Parser};
use parser::parse_xml;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version=crate_version!())]
#[command(about = "Extract texts from Wikipedia dump (.bz2) into jsonl.bz2", long_about = None)]
struct Cli {
    /// Path to the Wikipedia dump file (.bz2)
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,
}
fn main() {
    let cli = Cli::parse();

    let input = cli.input.to_str().unwrap();
    parse_xml(input.to_string()).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_verify_cli() {
        use clap::CommandFactory;
        super::Cli::command().debug_assert();
    }
}
