use crate::{
    error::ShukaError,
    types::{ContractMetadata, ParsedSourceBundle, RawExplorerResponse, SourceFile}
};

use serde_json::Value;


pub fn parse_source(source: &RawExplorerResponse) -> Result<ParsedSourceBundle, ShukaError> {
    let parsed: Value = serde_json::from_str(&source.body)
        .map_err(|err| ShukaError::Parser(format!("failed to parse: {err}")))?;

    // Get the "result" field from parsed JSON
    let result_value = parsed
        .get("result")
        .ok_or_else(|| ShukaError::Parser("missing result field".to_string()))?;

    // Check for array
    let result_array = result_value
        .as_array()
        .ok_or_else(|| ShukaError::Parser("result field is not an array".to_string()))?;

    // Get the 1st entry in the array
    let contract_entry = result_array
        .first()
        .ok_or_else(|| ShukaError::Parser("result array is empty".to_string()))?;

    // Get the `SourceCode`` field
    let source_code = contract_entry
        .get("SourceCode")
        .ok_or_else(|| ShukaError::Parser("`SourceCode` field is empty".to_string()))?
        .as_str()
        .ok_or_else(|| ShukaError::Parser("`SourceCode` is not a string".to_string()))?;

    // Get `ContractName` field
    let contract_name = contract_entry
        .get("ContractName")
        .ok_or_else(|| ShukaError::Parser("`ContractName` field is empty".to_string()))?
        .as_str()
        .ok_or_else(|| ShukaError::Parser("`ContractName` is not a string".to_string()))?;

    // Get `CompilerVersion` field
    let compiler_version = contract_entry
        .get("CompilerVersion")
        .ok_or_else(|| ShukaError::Parser("`CompilerVersion` filed is empty".to_string()))?
        .as_str()
        .ok_or_else(|| ShukaError::Parser("`CompilerVersion` is not a string".to_string()))?;

    println!("{}", source_code);

    Ok(ParsedSourceBundle {
        files: vec![
            SourceFile {
                path: format!("{}.sol", contract_name).into(), // @todo think about few contracts
                content: source_code.into(),
        },
        ],
        metadata: Some(ContractMetadata {
            contract_name: contract_name.into(),
            compiler_version: compiler_version.into(),
        }),
    })
}