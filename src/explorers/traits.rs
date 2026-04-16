use crate::{
    error::ShukaError,
    types::{FetchRequest, RawExplorerResponse},
};

pub trait SourceExplorer {
    fn fetch(&self, request: &FetchRequest) -> Result<RawExplorerResponse, ShukaError>;
}
