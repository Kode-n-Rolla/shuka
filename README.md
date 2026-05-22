# shuka

![shuka-banner](https://raw.githubusercontent.com/Kode-n-Rolla/shuka/main/assets/shuka-banner.png)

![Rust](https://img.shields.io/badge/Rust-CLI-orange)
![Status](https://img.shields.io/badge/status-early%20development-blue)
[![License](https://img.shields.io/crates/l/shuka)](https://spdx.org/licenses/MIT.html)
![Crates.io](https://img.shields.io/crates/v/shuka)
[![Docs](https://docs.rs/shuka/badge.svg)](https://docs.rs/shuka)

## About

`shuka` is a CLI tool for fetching verified smart contract source code from blockchain explorers and saving the full source tree locally.

It is built for workflows where manually copying verified source files from explorer pages is slow, repetitive, and error-prone.

The project keeps explorer-specific fetching, parsing, and filesystem storage separated so new explorers can be added without rewriting the whole pipeline.

## Links

- [Crates.io](https://crates.io/crates/shuka)
- [Documentation](https://docs.rs/shuka)

## Features

- Fetch verified smart contract source code by address.
- Save raw explorer responses for inspection.
- Save single-file and multi-file Solidity source trees.
- Keep explorer-specific API logic separate from parsing and storage.

## Supported Explorers

| Explorer | Network | API key | Chain ID |
|----------|----------|-------:|---------:|
| [Etherscan](https://etherscan.io/) | Ethereum Mainnet | Required | Required - 1 |
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

```bash
cargo install shuka
```

You still need to provide an Etherscan API key when using the Ethereum explorer.

`shuka` reads the key from the `ETHEREUM_API_KEY` environment variable. You can provide it in two ways:
1. export/set the variable in your terminal
2. create a `.env` file in the directory where you run shuka

Battlechain does not require an API key.

## Configuration

`shuka` does not store API keys. Provide them through environment variables or a local `.env` file in your working directory.

### Ethereum / Etherscan v2

Ethereum uses the Etherscan v2 API and requires an API key and `--chain-id` for usage.

Create a local `.env` file:

```bash
ETHEREUM_API_KEY=your_key_here
```

### Battlechain Testnet

Battlechain does not require:

- API key
- chain id

### After `cargo install`

#### Linux / macOS

Set the key for the current terminal session:

```bash
export ETHEREUM_API_KEY="your_key_here"
```

Then run:

```bash
shuka fetch ethereum 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain-id 1
```

To make it persistent, add it to your shell config:

```bash
echo 'export ETHEREUM_API_KEY="your_key_here"' >> ~/.bashrc
source ~/.bashrc
```

For Zsh users:

```bash
echo 'export ETHEREUM_API_KEY="your_key_here"' >> ~/.zshrc
source ~/.zshrc
```

#### Windows PowerShell

Set the key for the current PowerShell session:

```powershell
$env:ETHEREUM_API_KEY="your_key_here"
```

Then run:

```powershell
shuka fetch ethereum 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain-id 1
```

To make it persistent for your user account:

```powershell
[Environment]::SetEnvironmentVariable("ETHEREUM_API_KEY", "your_key_here", "User")
```

After setting it permanently, open a new PowerShell window.

#### Using a `.env` file

You can also create a `.env` file in the directory where you run `shuka`:

```bash
ETHEREUM_API_KEY=your_key_here
```
Example:

```bash
mkdir contract-sources
cd contract-sources
echo 'ETHEREUM_API_KEY=your_key_here' > .env
shuka fetch ethereum 0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48 --chain-id 1
```

## Usage

```bash
shuka --help
```
![general-help](https://github.com/Kode-n-Rolla/shuka/blob/main/assets/shuka-general-help.png)

```bash
shuka fetch --help
```
![fetch-help](https://github.com/Kode-n-Rolla/shuka/blob/main/assets/shuka-fetch-help.png)

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
