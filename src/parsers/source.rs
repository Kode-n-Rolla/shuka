use crate::{
    error::ShukaError,
    types::{ContractMetadata, ParsedSourceBundle, RawExplorerResponse, SourceFile},
};

use serde_json::Value;

/// Parses an explorer response into normalized source files and metadata.
///
/// The current parser supports Etherscan-like responses where `result` is an
/// array and each contract entry includes `SourceCode`, `ContractName`, and
/// `CompilerVersion`.
pub fn parse_source(source: &RawExplorerResponse) -> Result<ParsedSourceBundle, ShukaError> {
    let parsed: Value = serde_json::from_str(&source.body)
        .map_err(|err| ShukaError::Parser(format!("failed to parse: {err}")))?;

    let contract_entry = get_first_contract_entry(&parsed)?;

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

    let normalized_source_code = normalize_source_code(trimmed_source_code);

    // Get the `SourceCode` field for multi contracts
    let files = if normalized_source_code.starts_with("{") {
        parse_multi_file_source(&normalized_source_code)?
    } else {
        parse_single_file_source(contract_name, source_code)
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

fn get_first_contract_entry(value: &Value) -> Result<&Value, ShukaError> {
    let result_value = value
        .get("result")
        .ok_or_else(|| ShukaError::Parser("missing result field".to_string()))?;

    let result_array = result_value
        .as_array()
        .ok_or_else(|| ShukaError::Parser("result field is not an array".to_string()))?;

    result_array
        .first()
        .ok_or_else(|| ShukaError::Parser("result array is empty".to_string()))
}

fn normalize_source_code(source_code: &str) -> String {
    if source_code.starts_with("{{") && source_code.ends_with("}}") {
        // Remove the outer double braces
        source_code[1..source_code.len() - 1].to_string()
    } else {
        source_code.to_string()
    }
}

fn parse_multi_file_source(source_code: &str) -> Result<Vec<SourceFile>, ShukaError> {
    let parsed_multi_contracts: Value = serde_json::from_str(source_code)
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

    let mut files = Vec::new();

    for (file_name, file_value) in file_map {
        let content = get_required_string(file_value, "content")?;

        files.push(SourceFile {
            path: file_name.into(),
            content: content.into(),
        });
    }

    Ok(files)
}

fn parse_single_file_source(contract_name: &str, source_code: &str) -> Vec<SourceFile> {
    vec![SourceFile {
        path: format!("{contract_name}.sol").into(),
        content: source_code.into(),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_file_source() {
        let raw = RawExplorerResponse {
            body: r#"
            {
                "status": "1",
                "message": "OK",
                "result": [
                    {
                        "SourceCode": "pragma solidity ^0.8.0;\ncontract Token {}",
                        "ContractName": "Token",
                        "CompilerVersion": "v0.8.20+commit.a1b79de6"
                    }
                ]
            }
            "#
            .to_string(),
        };

        let bundle = parse_source(&raw).expect("single-file source should parse");

        assert_eq!(bundle.files.len(), 1);
        assert_eq!(bundle.files[0].path, std::path::PathBuf::from("Token.sol"));
        assert!(bundle.files[0].content.contains("contract Token"));
    }

    #[test]
    fn parses_multi_file_source() {
        let source_code = r#"{{"sources":{"src/A.sol":{"content":"pragma solidity ^0.8.0;\ncontract A {}"},"src/B.sol":{"content":"pragma solidity ^0.8.0;\ncontract B {}"}}}}"#;

        let raw = RawExplorerResponse {
            body: format!(
                r#"{{
                    "status": "1",
                    "message": "OK",
                    "result": [
                        {{
                            "SourceCode": {source_code:?},
                            "ContractName": "A",
                            "CompilerVersion": "v0.8.20+commit.a1b79de6"
                        }}
                    ]
                }}"#
            ),
        };

        let bundle = parse_source(&raw).expect("multi-file source should parse");

        assert_eq!(bundle.files.len(), 2);

        let paths: Vec<_> = bundle.files.iter().map(|file| file.path.clone()).collect();

        assert!(paths.contains(&std::path::PathBuf::from("src/A.sol")));
        assert!(paths.contains(&std::path::PathBuf::from("src/B.sol")));
    }
}
