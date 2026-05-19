use crate::{
    error::ShukaError,
    types::{ContractMetadata, ParsedSourceBundle, RawExplorerResponse, SourceFile},
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

    // Get the `SourceCode` field for 1 contract
    let source_code = get_required_string(contract_entry, "SourceCode")?;

    let trimmed_source_code = source_code.trim();

    if trimmed_source_code.is_empty() {
        return Err(ShukaError::Parser(
            "contract source code is empty".to_string(),
        ));
    }

    // Get `ContractName` field
    let contract_name = get_required_string(contract_entry, "ContractName")?;

    if contract_name.is_empty() {
        return Err(ShukaError::Parser("contract name is empty".to_string()));
    }

    // Get `CompilerVersion` field
    let compiler_version = get_required_string(contract_entry, "CompilerVersion")?;

    let normalized_source_code =
        if trimmed_source_code.starts_with("{{") && trimmed_source_code.ends_with("}}") {
            // Remove the outer double braces
            trimmed_source_code[1..trimmed_source_code.len() - 1].to_string()
        } else {
            trimmed_source_code.to_string()
        };

    // Get the `SourceCode` field for multi contracts
    let files = if trimmed_source_code.starts_with("{") {
        let parsed_multi_contracts: Value = serde_json::from_str(&normalized_source_code)
            .map_err(|err| ShukaError::Parser(format!("failed to parse multi contracts: {err}")))?;

        let file_map = if let Some(sources_value) = parsed_multi_contracts.get("sources") {
            sources_value
                .as_object()
                .ok_or_else(|| ShukaError::Parser("sources field is not an object".to_string()))?
        } else {
            parsed_multi_contracts.as_object().ok_or_else(|| {
                ShukaError::Parser("structured SourceCode file map not found".to_string())
            })?
        };

        let mut contracts = Vec::new();
        for (file_name, file_value) in file_map {
            let content = file_value
                .get("content")
                .ok_or_else(|| ShukaError::Parser("file content field is missing".to_string()))?
                .as_str()
                .ok_or_else(|| ShukaError::Parser("file content is not a string".to_string()))?;

            contracts.push(SourceFile {
                path: file_name.into(),
                content: content.into(),
            });
        }

        contracts
    } else {
        // single contract push
        let contract = vec![SourceFile {
            path: format!("{}.sol", contract_name).into(),
            content: source_code.into(),
        }];
        contract
    };

    Ok(ParsedSourceBundle {
        files,
        metadata: Some(ContractMetadata {
            contract_name: contract_name.into(),
            compiler_version: compiler_version.into(),
        }),
    })
}

fn get_required_string<'a>(value: &'a Value, field_name: &str) -> Result<&'a str, ShukaError> {
    value
        .get(field_name)
        .ok_or_else(|| ShukaError::Parser(format!("`{field_name}` field is missing")))?
        .as_str()
        .ok_or_else(|| ShukaError::Parser(format!("`{field_name}` is not a string")))
}
