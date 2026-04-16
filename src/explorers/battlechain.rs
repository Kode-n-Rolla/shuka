use crate::explorers::traits::SourceExplorer;
use crate::{
    error::ShukaError,
    types::{FetchRequest, RawExplorerResponse},
};

pub struct BattlechainExplorer;

impl SourceExplorer for BattlechainExplorer {
    fn fetch(&self, _request: &FetchRequest) -> Result<RawExplorerResponse, ShukaError> {
        Ok(RawExplorerResponse {
            body: "Battlechain response".to_string()
        })
    }
}