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
| Ethereum / Etherscan v2 | Ethereum Mainnet | Required | Required |
| Battlechain Explorer | Battlechain Testnet | Not required | Not required |

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

Adding an explorer means answering two separate questions:

1. How do we fetch the raw source-code response?
2. How do we parse that raw response into `ParsedSourceBundle`?

Do not treat those as one problem.

### 1. Check API Behavior First

Before writing code, gather:

- endpoint URL
- required query parameters
- whether an API key is required
- whether chain id is required
- one known verified contract address
- one sample raw response

Save request examples and raw responses locally while developing. This avoids
designing the parser blindly.

### 2. Add Enum Support

Update:

- `src/types.rs`
- `src/cli.rs`

Make sure:

- `ExplorerKind` has the new explorer.
- `CliExplorer` has the new explorer.
- `CliExplorer` maps correctly into `ExplorerKind`.

### 3. Create the Explorer Adapter

Add:

```text
src/explorers/<new_explorer>.rs
```

The adapter should:

1. implement `SourceExplorer`
2. build the explorer-specific request
3. send the HTTP request
4. validate HTTP status
5. read the body as text
6. return `RawExplorerResponse { body }`

If the explorer requires `--chain-id`, validate it in the adapter and return
`ShukaError::Cli` when it is missing. If the explorer does not use chain id,
leave it unused.

Do not parse contract files inside the adapter.

### 4. Register the Explorer

Update:

- `src/explorers/mod.rs`
- `src/app.rs`
- `src/storage/writer.rs`

The app layer should only choose the right adapter and keep the rest of the
pipeline unchanged.

Storage needs an explorer directory name so default output remains:

```text
contracts/<explorer>/<address>/
```

### 5. Test Fetch Before Parser Changes

Before changing parser behavior, make sure:

- raw fetch succeeds
- `raw_response.json` is saved

If fetch works but parsing fails, the next fix belongs in the parser, not in the
adapter.

## Handling Other Parse Cases

The current parser assumes an Etherscan-like response envelope:

- top-level JSON object
- `result` field exists
- `result` is an array
- first entry in `result` contains:
  - `SourceCode`
  - `ContractName`
  - `CompilerVersion`

`SourceCode` is handled in two formats:

1. plain string source
2. structured multi-file source

When a new explorer response fails to parse, inspect `raw_response.json` and
identify what changed:

- top-level response envelope
- contract entry field names
- `SourceCode` format
- metadata field names

Fix parsing in the smallest possible parser branch. Do not fix parser problems
in storage or explorer code if the raw response is already saved correctly.

If a future explorer introduces a truly different shape, consider adding
explorer-specific parser helpers such as:

- `parse_etherscan_like_source(...)`
- `parse_solscan_source(...)`

Only add that split when real response formats justify it.

## ToDo
- [ ] add badges after publish to crates
![Crates.io](https://img.shields.io/crates/v/shuka)
![Docs.rs](https://img.shields.io/docsrs/shuka)
- [ ] add MIT license
- [ ] move full instractions (add new exp and parsing) to `docs/` and replace to short version
