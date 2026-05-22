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
