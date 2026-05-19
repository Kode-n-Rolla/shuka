use crate::explorers::traits::SourceExplorer;
use crate::{
    error::ShukaError,
    explorers::{battlechain, ethereum},
    parsers::source::parse_source,
    storage::writer::{write_raw_response, write_source_files},
    types::{ExplorerKind, FetchOutcome, FetchRequest},
};

/// Runs the complete fetch pipeline for a single contract address.
///
/// The app layer selects the explorer adapter, saves the raw response, parses
/// source files, and delegates filesystem writes to storage.
pub fn run_fetch(request: FetchRequest) -> Result<FetchOutcome, ShukaError> {
    let raw_response = match &request.explorer {
        ExplorerKind::Ethereum => {
            let explorer = ethereum::EthereumExplorer;
            explorer.fetch(&request)
        }
        ExplorerKind::Battlechain => {
            let explorer = battlechain::BattlechainExplorer;
            explorer.fetch(&request)
        }
    }?;

    write_raw_response(&request, &raw_response)?;

    let parsed_bundle = parse_source(&raw_response)?;

    let save_result = write_source_files(&request, &parsed_bundle)?;

    Ok(FetchOutcome {
        output_path: save_result.output_path,
        files_written: save_result.files_written,
    })
}
