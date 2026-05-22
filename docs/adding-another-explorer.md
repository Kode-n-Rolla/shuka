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

If fetch works but parsing fails, the next fix belongs in the [parser](https://github.com/Kode-n-Rolla/shuka/blob/main/docs/parser-notes.md), not in the
adapter.
