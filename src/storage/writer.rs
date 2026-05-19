use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::{
    error::ShukaError,
    types::{ExplorerKind, FetchRequest, ParsedSourceBundle, RawExplorerResponse, SaveResult},
};

pub fn write_source_files(
    request: &FetchRequest,
    bundle: &ParsedSourceBundle,
) -> Result<SaveResult, ShukaError> {
    let output_path = resolve_output_dir(request);

    prepare_directory(&output_path)?;

    for file in &bundle.files {
        validate_source_path(&file.path)?;

        let full_path = output_path.join(&file.path);
        if let Some(parent) = full_path.parent() {
            prepare_directory(parent)?
        }

        fs::write(&full_path, &file.content).map_err(|err| {
            ShukaError::Storage(format!("failed to write to {}: {err}", full_path.display()))
        })?;
    }

    Ok(SaveResult {
        output_path,
        files_written: bundle.files.len(),
    })
}

pub fn write_raw_response(
    request: &FetchRequest,
    raw_response: &RawExplorerResponse,
) -> Result<PathBuf, ShukaError> {
    let file_name = String::from("raw_response.json");
    let out_path = resolve_output_dir(request);
    let full_path = out_path.join(file_name);

    prepare_directory(&out_path)?;

    fs::write(&full_path, &raw_response.body).map_err(|err| {
        ShukaError::Storage(format!("failed to write to {}: {err}", full_path.display()))
    })?;

    Ok(full_path)
}

// Helpers
fn resolve_output_dir(request: &FetchRequest) -> PathBuf {
    match &request.output_dir {
        Some(dir) => dir.clone(),
        None => PathBuf::from("contracts")
            .join(explorer_dir_name(request))
            .join(&request.address),
    }
}

fn prepare_directory(path: &Path) -> Result<(), ShukaError> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(ShukaError::Storage(format!(
                "failed to create {}: {err}",
                path.display()
            )));
        }
    }
}

fn validate_source_path(path: &Path) -> Result<(), ShukaError> {
    if path.as_os_str().is_empty() {
        return Err(ShukaError::Storage("source file path is empty".to_string()));
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(ShukaError::Storage(format!(
                    "source file path cannot contain '..': {}",
                    path.display()
                )));
            }
            Component::RootDir | Component::Prefix(_) => {
                return Err(ShukaError::Storage(format!(
                    "source file path must be relative: {}",
                    path.display()
                )));
            }
        }
    }

    Ok(())
}

fn explorer_dir_name(request: &FetchRequest) -> &'static str {
    match request.explorer {
        ExplorerKind::Ethereum => "ethereum",
        ExplorerKind::Battlechain => "battlechain",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_relative_source_path() {
        let path = Path::new("src/Token.sol");

        let result = validate_source_path(path);

        assert!(result.is_ok());
    }

    #[test]
    fn rejects_empty_source_path() {
        let path = Path::new("");

        let result = validate_source_path(path);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_parent_dir_source_path() {
        let path = Path::new("../Token.sol");

        let result = validate_source_path(path);

        assert!(result.is_err());
    }

    #[test]
    fn rejects_absolute_source_path() {
        let path = Path::new("/tmp/Token.sol");

        let result = validate_source_path(path);

        assert!(result.is_err());
    }

    #[test]
    fn resolves_default_output_dir_by_explorer_and_address() {
        let request = FetchRequest {
            explorer: ExplorerKind::Battlechain,
            address: "0xabc".to_string(),
            chain_id: None,
            output_dir: None,
        };

        let output_dir = resolve_output_dir(&request);

        assert_eq!(
            output_dir,
            PathBuf::from("contracts").join("battlechain").join("0xabc")
        );
    }

    #[test]
    fn respects_custom_output_dir() {
        let request = FetchRequest {
            explorer: ExplorerKind::Ethereum,
            address: "0xabc".to_string(),
            chain_id: Some(1),
            output_dir: Some(PathBuf::from("custom")),
        };

        let output_dir = resolve_output_dir(&request);

        assert_eq!(output_dir, PathBuf::from("custom"));
    }
}
