# Lambda Nodes Agent

The Agent is the backend service for Lambda Nodes, responsible for managing pipelines, nodes, and routes. It is built with Rust and uses PostgreSQL as its database.

## Features

- High-performance backend powered by Rust.
- PostgreSQL integration for data persistence.
- RESTful API for communication with the frontend.

## Requirements

- Rust
- PostgreSQL

## Environment Variables

The following environment variables are required to run the Agent:

- `DB_PORT`: The port for the PostgreSQL database (default: `5433`).
- `LOG_LEVEL`: The logging level (e.g., `debug`, `info`, `warn`, `error`).
- `CLIENT_ADDRESS`: The frontend address (default: `http://localhost:5173`).

## Installation

1. Navigate to the `agent` directory:
   ```bash
   cd agent
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Running the Agent

1. Initialize and seed the database

Execute `sql/schema.sql` and `sql/seed.sql` in your PostgreSQL database to setup database.

2. Start the backend server:
   ```bash
   cargo run
   ```

3. The backend will be available at `http://localhost:3000`.

## Building for Production

1. Build the release version:
   ```bash
   cargo build --release
   ```

2. Deploy the binary to your server.
