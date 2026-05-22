# shuka

![shuka-banner](https://raw.githubusercontent.com/Kode-n-Rolla/shuka/main/assets/shuka-banner.png)

![Rust](https://img.shields.io/badge/Rust-CLI-orange)
![Status](https://img.shields.io/badge/status-early%20development-blue)
![License](https://img.shields.io/github/license/Kode-n-Rolla/shuka)

## About

`shuka` is a CLI tool for fetching verified smart contract source code from blockchain explorers and saving the full source tree locally.

It is built for workflows where manually copying verified source files from explorer pages is slow, repetitive, and error-prone.

The project keeps explorer-specific fetching, parsing, and filesystem storage separated so new explorers can be added without rewriting the whole pipeline.

## Features

- Fetch verified smart contract source code by address.
- Save raw explorer responses for inspection.
- Save single-file and multi-file Solidity source trees.
- Keep explorer-specific API logic separate from parsing and storage.

## Supported Explorers

| Explorer | Network | API key | Chain ID |
|----------|----------|-------:|---------:|
| [Etherscan](https://etherscan.io/) | Ethereum Mainnet | Required | Required |
| [Battlechain](https://explorer.testnet.battlechain.com/) | Battlechain Testnet | Not required | Not required |

## Installation

### Prerequisites

Install Rust and Cargo:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Check installation:

```bash
rustc --version
cargo --version
```

### Install from source

```bash
git clone https://github.com/Kode-n-Rolla/shuka.git
cd shuka
cargo install --path .
```

### Install with Cargo

TODO: this command will be available after publishing to crates.io.

```bash
cargo install shuka
```

## Configuration

### Ethereum / Etherscan v2

Ethereum uses the Etherscan v2 API and requires an API key.

Create a local `.env` file:

```bash
ETHEREUM_API_KEY=your_key_here
```

Ethereum also requires `--chain-id`. For Ethereum Mainnet, use:

```
--chain-id 1
```

### Battlechain Testnet

Battlechain does not require:

- API key
- chain id

## Usage

TODO - Show help: screenshot

```bash
shuka --help
shuka fetch --help
```

Fetch an Ethereum contract:

```bash
shuka fetch ethereum 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain-id 1
```

Fetch a Battlechain contract:

```bash
shuka fetch battlechain 0x526Ed31aAfbbe40f077D94CEAfDF20B5f99Bd7B1
```

Choose an output directory:

```bash
shuka fetch ethereum 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain-id 1 --out ./contracts/usdc
```

Print the optional banner:

```bash
shuka --with-banner fetch battlechain 0x526Ed31aAfbbe40f077D94CEAfDF20B5f99Bd7B1
```

By default, output is written to:

```text
contracts/<explorer>/<address>/
```

Each output directory contains:

- `raw_response.json`
- extracted source files

## Architecture

`shuka` is intentionally organized as a small pipeline where each stage has one responsibility.

The project follows a simple pipeline:

```text
FetchRequest
  -> Explorer Adapter
  -> RawExplorerResponse
  -> Parser
  -> ParsedSourceBundle
  -> Storage
  -> SaveResult
  -> FetchOutcome
```

Module responsibilities:

- CLI parses arguments, builds `FetchRequest`, and displays results.
- App orchestrates the pipeline.
- Explorer adapters contain API-specific fetch logic.
- Parser transforms raw explorer responses into normalized source files.
- Storage writes files to disk and validates source paths.
- Types define shared data models.
- Error defines the unified project error type.

Important boundaries:

- Explorer adapters should not parse source files.
- Parser should not write files.
- Storage should not know explorer API details.
- CLI should stay thin.

## Adding Another Explorer

Adding a new explorer usually means:

1. document the API behavior
2. add explorer enum support
3. implement a `SourceExplorer` adapter
4. register the adapter in the app layer
5. test raw response saving
6. update parser logic only if the raw response shape differs

See [docs/adding-another-explorer.md](https://github.com/Kode-n-Rolla/shuka/blob/main/docs/adding-another-explorer.md) for the full checklist.

## Parser Notes

The current parser handles Etherscan-like response envelopes and supports:

1. plain Solidity source
2. structured multi-file source

If a new explorer fails to parse but `raw_response.json` is saved correctly, the fix usually belongs in the parser, not in the explorer adapter or storage layer.
Full instructions 👉 [docs/parser-notes.md](https://github.com/Kode-n-Rolla/shuka/blob/main/docs/parser-notes.md)

## ToDo
- [ ] add badges after publish to crates
![Crates.io](https://img.shields.io/crates/v/shuka)
![Docs.rs](https://img.shields.io/docsrs/shuka)
