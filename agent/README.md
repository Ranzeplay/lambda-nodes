# Lambda Nodes Agent

The Agent is the backend service for Lambda Nodes, responsible for managing pipelines, nodes, and routes. It is built with Rust and uses PostgreSQL as its database.

## Features

- High-performance backend powered by Rust.
- PostgreSQL integration for data persistence.
- RESTful API for communication with the frontend.

## Requirements

- Rust (>= 1.70)
- PostgreSQL (>= 13.x)

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

1. Start the backend server:
   ```bash
   cargo run
   ```

2. The backend will be available at `http://localhost:3000`.

## Building for Production

1. Build the release version:
   ```bash
   cargo build --release
   ```

2. Deploy the binary to your server.

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.
