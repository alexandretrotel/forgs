# forgs

A tool to scan the organizations behind a repository's stargazers.

## Quick Start

```bash
forgs token set <github-token>
forgs scan owner/name
```

## Installation

```bash
cargo install forgs
```

## Usage

```bash
forgs scan owner/name
forgs scan https://github.com/owner/name
forgs scan owner/name other-owner/other-name
forgs scan --output results.json owner/name
forgs scan --output ./results owner/name other-owner/other-name
forgs token set <github-token>
forgs token delete
```

## Notes

- Without `--output`, results are printed to stdout in a human-readable terminal format.
- `scan` accepts either `owner/name` or a full GitHub repository URL.
- If `--output` points to a file, all results are written there as one JSON array.
- If `--output` points to a directory, one file per repo is written as `owner-repo.json`.
- Scans can run without a token, but GitHub rate limits may cause errors more quickly.

## License

GPL-3.0. See [LICENSE](./LICENSE).
