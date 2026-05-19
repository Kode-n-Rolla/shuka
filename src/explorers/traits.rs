use crate::{
    error::ShukaError,
    types::{FetchRequest, RawExplorerResponse},
};

/// Common interface for explorer adapters that can fetch contract source data.
pub trait SourceExplorer {
    /// Fetches raw source-code data for the given request.
    fn fetch(&self, request: &FetchRequest) -> Result<RawExplorerResponse, ShukaError>;
}
