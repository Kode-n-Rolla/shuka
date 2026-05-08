use std::fs;
use std::path::{Path, PathBuf};

use crate::{
    error::ShukaError,
    types::{FetchRequest, ParsedSourceBundle, SaveResult, RawExplorerResponse}
};


pub fn write_source_files(
    request: &FetchRequest,
    bundle: &ParsedSourceBundle) -> Result<SaveResult, ShukaError> {

        let output_path = resolve_output_dir(request);

        prepare_directory(&output_path)?;

        for file in &bundle.files {
            let full_path = output_path.join(&file.path);
            if let Some(parent) = full_path.parent() {
                prepare_directory(parent)?
            }

            fs::write(&full_path, &file.content)
                .map_err(|err| ShukaError::Storage(format!("failed to write to {}: {err}", full_path.display())))?;
        }

        Ok(SaveResult {
            output_path,
            files_written: bundle.files.len()
        })
}

pub fn write_raw_response(request: &FetchRequest, raw_response: &RawExplorerResponse) -> Result<PathBuf, ShukaError> {
    let file_name = String::from("raw_response.json");
    let out_path = resolve_output_dir(request);
    let full_path= out_path.join(file_name);

    prepare_directory(&out_path)?;

    fs::write(&full_path, &raw_response.body)
        .map_err(|err| ShukaError::Storage(format!("failed to write to {}: {err}", full_path.display())))?;

    Ok(full_path)
}

// Helpers
fn resolve_output_dir(request: &FetchRequest) -> PathBuf {
    let save_dir = match &request.output_dir {
        Some(dir) => dir.clone(),
        None => PathBuf::from("contracts/")
    };

    save_dir
}

fn prepare_directory(path: &Path) -> Result<(), ShukaError> {
    match fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(ShukaError::Storage(format!("failed to create {}: {err}", path.display())));
        }
    }
}