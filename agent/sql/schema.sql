BEGIN;

CREATE TYPE log_level AS ENUM ('info', 'warn', 'error');

CREATE TABLE IF NOT EXISTS logs
(
    id        SERIAL PRIMARY KEY,
    level     log_level                NOT NULL,
    message   TEXT                     NOT NULL,
    create_at TIMESTAMP WITH TIME ZONE not null default now()
);

CREATE INDEX IF NOT EXISTS idx_logs_create_at ON logs (create_at);

CREATE TABLE IF NOT EXISTS nodes
(
    id      UUID PRIMARY KEY NOT NULL,
    name    TEXT             NOT NULL,
    content JSON             NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_nodes_name ON nodes (name);

CREATE TYPE http_method AS ENUM ('GET', 'POST', 'PUT', 'DELETE', 'PATCH');

CREATE TABLE IF NOT EXISTS pipelines
(
    id      UUID PRIMARY KEY NOT NULL,
    name    TEXT             NOT NULL,
    content JSON             NOT NULL,
    method  http_method      NOT NULL,
    url     TEXT             NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_pipelines_url ON pipelines (url);

COMMIT;
