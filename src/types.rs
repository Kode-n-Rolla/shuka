use std::path::PathBuf;

/// Supported blockchain explorer adapters.
#[derive(Debug)]
pub enum ExplorerKind {
    /// Battlechain testnet block explorer.
    Battlechain,
    /// Etherscan-compatible Ethereum explorer.
    Ethereum,
}

/// User request passed from the CLI into the application pipeline.
#[derive(Debug)]
pub struct FetchRequest {
    /// Explorer adapter to use for the fetch.
    pub explorer: ExplorerKind,
    /// Contract address to fetch.
    pub address: String,
    /// Optional chain id. Required by Ethereum/Etherscan v2, unused by Battlechain.
    pub chain_id: Option<u32>,
    /// Optional output directory. If omitted, storage chooses a default path.
    pub output_dir: Option<PathBuf>,
}

/// Raw explorer response returned by an adapter before parsing.
#[derive(Debug)]
pub struct RawExplorerResponse {
    /// Raw response body as text.
    pub body: String,
    // pub status: Option<String>,
    // pub message: Option<String>,
}

/// A normalized source file ready to be written to disk.
#[derive(Debug)]
pub struct SourceFile {
    /// Relative path of the source file inside the output directory.
    pub path: PathBuf,
    /// Source file contents.
    pub content: String,
}

/// Normalized source bundle produced by the parser.
#[derive(Debug)]
pub struct ParsedSourceBundle {
    /// Source files extracted from the explorer response.
    pub files: Vec<SourceFile>,
    /// Optional normalized contract metadata.
    pub metadata: Option<ContractMetadata>,
}

/// Minimal contract metadata extracted from explorer responses.
#[derive(Debug)]
pub struct ContractMetadata {
    /// Contract name reported by the explorer.
    pub contract_name: String,
    /// Compiler version reported by the explorer.
    pub compiler_version: String,
}

/// Result of writing a parsed source bundle to disk.
#[derive(Debug)]
pub struct SaveResult {
    /// Directory where files were written.
    pub output_path: PathBuf,
    /// Number of source files written.
    pub files_written: usize,
}

/// Final outcome returned by the application pipeline.
#[derive(Debug)]
pub struct FetchOutcome {
    /// Directory where files were written.
    pub output_path: PathBuf,
    /// Number of source files written.
    pub files_written: usize,
}
