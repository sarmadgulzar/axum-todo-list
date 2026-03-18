# axum-todo-list

A small JSON todo API built with Rust, Axum, SQLx, and SQLite.

This project is part of my Rust/Axum portfolio work and focuses on the backend fundamentals of building an HTTP API: routing, shared application state, request parsing, error handling, database access, and migrations.

## Overview

The app starts an Axum server on `127.0.0.1:8080`, creates a local SQLite database file named `todo.db` if it does not exist, and runs the SQL migrations automatically on startup.

Each todo includes:

- `id` as a UUID v7
- `title`
- `completed`
- `created_at`
- `updated_at`

## Features

- Create a todo
- List all todos
- Fetch a single todo by ID
- Delete a todo
- Mark a todo as complete
- Mark a todo as incomplete
- Health check endpoint
- Automatic SQLite migrations on startup

## Tech Stack

- Rust 2024 edition
- [Axum](https://github.com/tokio-rs/axum)
- [Tokio](https://tokio.rs/)
- [SQLx](https://github.com/launchbadge/sqlx)
- SQLite
- Serde
- Chrono
- UUID v7

## API Endpoints

Base URL: `http://127.0.0.1:8080`

| Method | Route | Description |
| --- | --- | --- |
| `GET` | `/` | Simple root endpoint |
| `GET` | `/health` | Health check |
| `GET` | `/todos` | List all todos |
| `POST` | `/todos` | Create a todo |
| `GET` | `/todos/{id}` | Get a single todo |
| `DELETE` | `/todos/{id}` | Delete a todo |
| `POST` | `/todos/{id}/mark-complete` | Mark a todo complete |
| `POST` | `/todos/{id}/mark-incomplete` | Mark a todo incomplete |

## Request / Response Examples

Create a todo:

```bash
curl -X POST http://127.0.0.1:8080/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"Build an Axum portfolio project"}'
```

Example response:

```json
{
  "id": "0195ad6b-0d9a-7c7a-a7a1-7a0d5a9cf012",
  "title": "Build an Axum portfolio project",
  "completed": false,
  "created_at": "2026-03-18T20:15:41Z",
  "updated_at": "2026-03-18T20:15:41Z"
}
```

List todos:

```bash
curl http://127.0.0.1:8080/todos
```

Get one todo:

```bash
curl http://127.0.0.1:8080/todos/<todo-id>
```

Mark complete:

```bash
curl -X POST http://127.0.0.1:8080/todos/<todo-id>/mark-complete
```

Delete a todo:

```bash
curl -X DELETE http://127.0.0.1:8080/todos/<todo-id>
```

## Running Locally

### Prerequisites

- Rust toolchain installed
- Cargo

### Start the app

```bash
git clone git@github.com:sarmadgulzar/axum-todo-list.git
cd axum-todo-list
cargo run
```

The server will start on `127.0.0.1:8080`.

On startup the application will:

1. create `todo.db` if it does not already exist
2. apply the SQL migrations from the `migrations/` directory
3. start serving requests

## Testing

Run the test suite with:

```bash
cargo test
```

At the moment, the automated tests cover the root and health endpoints.

## Project Structure

```text
src/
  handlers.rs    # request handlers
  router.rs      # route definitions
  models.rs      # database response models
  schemas.rs     # request payloads
  state.rs       # shared app state
  error.rs       # application error mapping
  tests/         # endpoint tests
migrations/      # SQLite migration files
```

## What This Project Demonstrates

- Building an API with Axum routing and extractors
- Managing shared state with a database pool
- Executing SQL queries with SQLx
- Returning typed JSON responses
- Mapping database failures into HTTP responses
- Organizing a Rust web service into small, focused modules

## Current Limitations

- The database path is currently hardcoded to `sqlite:todo.db`
- The server host and port are currently hardcoded to `127.0.0.1:8080`
- There is no authentication or frontend
- Test coverage is still minimal

## Possible Next Steps

- Add update/edit support for todo titles
- Add pagination and sorting to `GET /todos`
- Move configuration into environment variables
- Add integration tests for the todo CRUD routes
- Add request validation and better error response bodies

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE).

## Note on AI Usage

The application code in this project was built without AI assistance.

The only AI-assisted part of this repository was the generation of this README file.
