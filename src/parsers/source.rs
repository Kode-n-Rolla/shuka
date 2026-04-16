use crate::{
    error::ShukaError,
    types::{ContractMetadata, ParsedSourceBundle, RawExplorerResponse, SourceFile}};


pub fn parse_source(_source: &RawExplorerResponse) -> Result<ParsedSourceBundle, ShukaError> {
    Ok(ParsedSourceBundle {
        files: vec![
            SourceFile {
                path: "Contract.sol".into(),
                content: "Placeholder content".to_string(),
        },
        ],
        metadata: Some(ContractMetadata {
            contract_name: "Contract name".to_string(),
            compiler_version: "0.8.29".to_string(),
        }),
    })
}