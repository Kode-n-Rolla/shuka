use std::path::PathBuf;

#[derive(Debug)]
pub enum ExplorerKind {
    Battlechain,
    Ethereum,
}

#[derive(Debug)]
pub struct FetchRequest {
    pub explorer: ExplorerKind,
    pub address: String,
    pub chain_id: u32,
    pub output_dir: Option<PathBuf>,
}

#[derive(Debug)]
pub struct RawExplorerResponse {
    pub body: String,
    // pub status: Option<String>,
    // pub message: Option<String>,
}

#[derive(Debug)]
pub struct SourceFile {
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug)]
pub struct ParsedSourceBundle {
    pub files: Vec<SourceFile>,
    pub metadata: Option<ContractMetadata>,
}

#[derive(Debug)]
pub struct ContractMetadata {
    pub contract_name: String,
    pub compiler_version: String,
}

#[derive(Debug)]
pub struct SaveResult {
    pub output_path: PathBuf,
    pub files_written: usize,
}

#[derive(Debug)]
pub struct FetchOutcome {
    pub output_path: PathBuf,
    pub files_written: usize,
}