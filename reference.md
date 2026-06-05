# Petstore API CLI Reference

Full command reference for `petstore-api`.

## Commands

- [`petstore-api auth`](#petstore-api-auth)
- [`petstore-api pets`](#petstore-api-pets)

---

### `petstore-api auth`

#### `petstore-api auth get-token`

Get an access token

`POST /auth/token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `petstore-api pets`

#### `petstore-api pets create-pet`

Create a pet

`POST /pets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `petstore-api pets get-pet`

Get a pet by ID

`GET /pets/{petId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--pet-id` | `string` | Yes | The ID of the pet to retrieve. |

#### `petstore-api pets list-pets`

List all pets

`GET /pets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--limit` | `integer (int32)` | No | Maximum number of pets to return. |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

