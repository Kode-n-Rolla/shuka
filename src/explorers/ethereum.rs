use crate::explorers::traits::SourceExplorer;
use crate::{
    error::ShukaError,
    types::{FetchRequest, RawExplorerResponse},
};


pub struct EthereumExplorer;

impl SourceExplorer for EthereumExplorer {
    fn fetch(&self, _request: &FetchRequest) -> Result<RawExplorerResponse, ShukaError> {
        Ok(RawExplorerResponse {
            body: "Ethereum response".to_string(),
         })
    }
}