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
forgs scan owner/name other-owner/other-name
forgs scan --output results.json owner/name
forgs token set <github-token>
forgs token delete
```

Scans can run without a token, but GitHub rate limits may cause errors more quickly.

## License

GPL-3.0. See [LICENSE](./LICENSE).
