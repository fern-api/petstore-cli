# Petstore API CLI

Command-line interface for the Petstore API.

## Table of contents

- [Installation](#installation)
- [Authentication](#authentication)
- [Quick start](#quick-start)
- [Usage](#usage)
- [Documentation](#documentation)
- [Advanced](#advanced)
  - [Common flags](#common-flags)
  - [Environment variables](#environment-variables)
  - [Output formats](#output-formats)
  - [Shell completion](#shell-completion)

## Installation

Install the CLI globally via npm:

```bash
npm install -g @fern-api/example-cli-petstore
```

Or run it directly without installing:

```bash
npx @fern-api/example-cli-petstore --help
```

### Build from source

If you prefer to build from source, install the [Rust toolchain](https://rustup.rs/) and run:

```bash
cargo build --release
./target/release/petstore-api --help
```

## Authentication

Set the following environment variable(s) before using the CLI:

```bash
export PETSTORE_TOKEN="<your token>"
```

A `.env` file in the working directory is also supported — the CLI auto-loads it on startup.

## Quick start

List available commands:

```bash
petstore-api --help
```

Call an API endpoint:

```bash
petstore-api <resource> <method>
```

Run `petstore-api <resource> --help` to see available methods for a resource.

## Usage

Every API resource appears as a subcommand (e.g. `petstore-api <resource> <method>`). Run `petstore-api <resource> --help` to see available methods.

Provide request parameters as flags or as JSON:

```bash
petstore-api <resource> <method> --json '{"key": "value"}'
```

## Documentation

See [reference.md](./reference.md) for the full command reference.

## Advanced

### Common flags

These flags are available on every operation:

| Flag | Description |
|------|-------------|
| `--dry-run` | Validate the request locally and print the HTTP request without sending it |
| `--json <JSON\|->` | Supply a request body as JSON (or `-` to read stdin) |
| `--params <JSON>` | Merge extra parameters as JSON (overrides individual flags) |
| `--format <json\|table\|yaml\|csv>` | Output format (default `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream results as NDJSON |
| `--page-limit <N>` | Max pages to fetch when auto-paginating (default `10`) |
| `-q, --quiet` | Suppress stdout output on success (errors still go to stderr) |

### Environment variables

| Variable | Description |
|----------|-------------|
| `PETSTORE_API_BASE_URL` | Override the API base URL |
| `PETSTORE_API_CA_BUNDLE` | Path to PEM file with extra trust roots (or `SSL_CERT_FILE`) |
| `PETSTORE_API_INSECURE=1` | Skip TLS verification (debugging only) |
| `PETSTORE_API_PROXY` | HTTP(S) proxy URL |
| `PETSTORE_API_TIMEOUT_SECS` | Total request timeout in seconds |

Standard environment variables (`HTTPS_PROXY` / `HTTP_PROXY` / `NO_PROXY` / `SSL_CERT_FILE`) are also honored.

### Output formats

Use the global `--format` flag to control output. Supported values: `json` (default), `table`, `yaml`, `csv`.

```bash
# Pipe JSON output through jq
petstore-api <resource> <method> --format json | jq

# Machine-readable catalog of every operation
petstore-api --help --format json | jq 'length'
```

### Shell completion

Generate shell completion scripts:

```bash
petstore-api completion <bash|zsh|fish|powershell>
```

