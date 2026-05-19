use std::path::PathBuf;

use crate::{
    app::run_fetch,
    error::ShukaError,
    types::{ExplorerKind, FetchRequest},
};
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "shuka")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(override_usage = "shuka fetch <EXPLORER> <ADDRESS> <CHAIN_ID> [OPTIONS]")]
    Fetch(FetchArgs),
}

#[derive(Debug, Args)]
pub struct FetchArgs {
    pub explorer: CliExplorer,
    pub address: String,
    pub chain_id: u32,
    #[arg(short, long)]
    pub out: Option<PathBuf>,
}

impl FetchArgs {
    pub fn into_request(self) -> FetchRequest {
        let request: FetchRequest = FetchRequest {
            explorer: self.explorer.into(),
            address: self.address,
            chain_id: self.chain_id,
            output_dir: self.out,
        };

        request
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum CliExplorer {
    Battlechain,
    Ethereum,
}

impl From<CliExplorer> for ExplorerKind {
    fn from(value: CliExplorer) -> Self {
        match value {
            CliExplorer::Battlechain => ExplorerKind::Battlechain,
            CliExplorer::Ethereum => ExplorerKind::Ethereum,
        }
    }
}

pub fn run() -> Result<(), ShukaError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Fetch(args) => {
            let request = args.into_request();
            let outcome = run_fetch(request)?;

            println!("Fetched source successfully");
            println!("Output directory: {}", outcome.output_path.display());
            println!("Files written: {}", outcome.files_written);

            Ok(())
        }
    }
}
