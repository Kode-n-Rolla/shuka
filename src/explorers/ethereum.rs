use crate::explorers::traits::SourceExplorer;
use crate::{
    error::ShukaError,
    types::{FetchRequest, RawExplorerResponse},
};

use dotenvy;
use reqwest::blocking;
use std::env;

const BASE_API_URL: &str = "https://api.etherscan.io/v2/api";

pub struct EthereumExplorer;

impl SourceExplorer for EthereumExplorer {
    fn fetch(&self, request: &FetchRequest) -> Result<RawExplorerResponse, ShukaError> {
        let key = get_api_key()?;

        // Prepare the request
        let client = blocking::Client::new();
        let chain_id = request
            .chain_id
            .ok_or_else(|| ShukaError::Cli("ethereum explorer requires --chain-id".to_string()))?
            .to_string();

        let response = client
            .get(BASE_API_URL)
            .query(&[
                ("apikey", key),
                ("chainid", chain_id),
                ("module", "contract".to_string()),
                ("action", "getsourcecode".to_string()),
                ("address", request.address.clone()),
            ])
            .send()
            .map_err(|err| {
                ShukaError::Explorer(format!("failed to send request to Etherscan: {err}"))
            })?;

        let status = response.status();
        if !status.is_success() {
            return Err(ShukaError::Explorer(format!(
                "Etherscan API returned unsuccessful status: {status}"
            )));
        }

        let body = response
            .text()
            .map_err(|err| ShukaError::Explorer(format!("Failed to handle body: {err}")))?;

        Ok(RawExplorerResponse { body })
    }
}

fn get_api_key() -> Result<String, ShukaError> {
    dotenvy::dotenv().ok();
    env::var("ETHEREUM_API_KEY")
        .map_err(|err| ShukaError::Config(format!("failed to read ETHEREUM_API_KEY: {err}")))
}
