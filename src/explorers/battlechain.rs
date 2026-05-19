use crate::explorers::traits::SourceExplorer;
use crate::{
    error::ShukaError,
    types::{FetchRequest, RawExplorerResponse},
};

use reqwest::blocking;

const BASE_API_URL: &str = "https://block-explorer-api.testnet.battlechain.com/api";

pub struct BattlechainExplorer;

impl SourceExplorer for BattlechainExplorer {
    fn fetch(&self, request: &FetchRequest) -> Result<RawExplorerResponse, ShukaError> {
        let client = blocking::Client::new();

        let response = client
            .get(BASE_API_URL)
            .query(&[
                ("module", "contract".to_string()),
                ("action", "getsourcecode".to_string()),
                ("address", request.address.clone()),
            ])
            .send()
            .map_err(|err| {
                ShukaError::Explorer(format!("failed to send request to Battlechain: {err}"))
            })?;

        let status = response.status();
        if !status.is_success() {
            return Err(ShukaError::Explorer(format!(
                "Battlechain API returned unsuccessful status: {status}"
            )));
        }

        let body = response
            .text()
            .map_err(|err| ShukaError::Explorer(format!("Failed to handle body: {err}")))?;

        Ok(RawExplorerResponse { body })
    }
}
