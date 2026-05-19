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
    #[arg(long, global = true)]
    pub with_banner: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(override_usage = "shuka fetch <EXPLORER> <ADDRESS> [OPTIONS]")]
    Fetch(FetchArgs),
}

#[derive(Debug, Args)]
pub struct FetchArgs {
    pub explorer: CliExplorer,
    pub address: String,
    #[arg(long)]
    pub chain_id: Option<u32>,
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

fn print_banner() {
    println!(
        r#"
       .__           __            
  _____|  |__  __ __|  | _______   
 /  ___/  |  \|  |  \  |/ /\__  \  
 \___ \|   Y  \  |  /    <  / __ \_
/____  >___|  /____/|__|_ \(____  /
     \/     \/           \/     \/ 
                                     
        "#
    );
}

pub fn run() -> Result<(), ShukaError> {
    let cli = Cli::parse();

    if cli.with_banner {
        print_banner();
    }

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
