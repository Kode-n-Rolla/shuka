//! Shuka fetches verified smart contract source code from blockchain explorers
//! and writes the recovered source tree to disk.
//!
//! The crate is organized as a small pipeline:
//! `FetchRequest -> Explorer Adapter -> RawExplorerResponse -> Parser -> Storage`.
//! CLI code should stay thin, explorer adapters should only fetch raw data, the
//! parser should normalize explorer responses, and storage should only handle
//! filesystem writes.

pub mod app;
pub mod cli;
pub mod error;
pub mod explorers;
pub mod parsers;
pub mod storage;
pub mod types;
