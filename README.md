# shuka

![shuka-banner](https://github.com/Kode-n-Rolla/shuka/blob/main/assets/shuka-banner.png)

`shuka` is a CLI tool for fetching verified smart contract source code
from blockchain explorers and saving the full source tree locally.

It is built for the workflow where manually copying verified source files from
explorer pages is slow and error-prone.

## Features

- Fetch verified smart contract source code by address.
- Save raw explorer responses for inspection.
- Save single-file and multi-file Solidity source trees.
- Support Ethereum through Etherscan v2.
- Support the Battlechain testnet explorer.
- Keep explorer-specific API logic separate from parsing and storage.

## Installation

From source:

```bash
cargo install --path .
```

After publishing to crates.io:

```bash
cargo install shuka
```

## Configuration

Ethereum uses the Etherscan v2 API and requires an API key:

```bash
ETHEREUM_API_KEY=your_key_here
```

You can put this in a local `.env` file while developing. `.env` is ignored by
git.

Battlechain does not require a chain id or API key.

## Usage

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

## Development Checks

Run formatting:

```bash
cargo fmt --check
```

Run tests:

```bash
cargo test
```

Build docs:

```bash
cargo doc --no-deps
```

Strict docs check:

```bash
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Package dry run:

```bash
cargo publish --dry-run
```

## Publishing Notes

Before publishing:

1. Ensure `Cargo.toml` metadata is correct.
2. Run `cargo fmt --check`.
3. Run `cargo test`.
4. Run `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps`.
5. Run `cargo publish --dry-run`.
6. Commit the release state.
7. Publish with `cargo publish`.

Publishing to crates.io is permanent for a version. If a bad version is
published, it can be yanked, but the uploaded version cannot be replaced.
