# Lambda Nodes

Lambda Nodes is a modern, full-stack application for managing pipelines, nodes, and routes.
It provides a user-friendly interface for creating, editing, and monitoring workflows.

## Requirements

- Node.js (>= 16.x)
- pnpm (>= 7.x)
- Docker (optional, for containerized deployment)
- PostgreSQL (>= 13.x)

## Environment Variables Documentation

### Database Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `DB_HOST` | PostgreSQL database host | `localhost` |
| `DB_PORT` | PostgreSQL database port | `5432` |
| `DB_NAME` | PostgreSQL database name | `lambda-nodes` |
| `DB_USER` | PostgreSQL database user | `postgres` |
| `DB_PASSWORD` | PostgreSQL database password | `postgres` |

### Logging Configuration

| Variable | Description | Default |
|----------|-------------|---------|
| `LOG_LEVEL` | Application logging level | `info` |
