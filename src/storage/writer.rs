use std::path::PathBuf;

use crate::{error::ShukaError, types::{FetchRequest, ParsedSourceBundle, SaveResult}};


pub fn write_source_files(
    request: &FetchRequest,
    bundle: &ParsedSourceBundle) -> Result<SaveResult, ShukaError> {

        let save_dir = match &request.output_dir {
            Some(dir) => dir.clone(),
            None => PathBuf::from("contracts/")
        };

        Ok(SaveResult {
            output_path: save_dir,
            files_written: bundle.files.len()
        })
}