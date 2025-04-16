BEGIN;

CREATE TYPE log_level AS ENUM ('info', 'warn', 'error');

CREATE TABLE IF NOT EXISTS logs
(
    id        SERIAL PRIMARY KEY,
    level     log_level                NOT NULL,
    category  TEXT                     NOT NULL,
    message   TEXT                     NOT NULL,
    create_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_logs_create_at ON logs (create_at);

CREATE TABLE IF NOT EXISTS nodes
(
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid() NOT NULL,
    is_internal BOOLEAN          DEFAULT FALSE NOT NULL,
    name        TEXT   NOT NULL,
    script      TEXT   NOT NULL,
    inputs      TEXT[] NOT NULL  DEFAULT '{}',
    outputs     TEXT[] NOT NULL  DEFAULT '{}'
);

CREATE INDEX IF NOT EXISTS idx_nodes_name ON nodes (name);

CREATE TABLE IF NOT EXISTS pipelines
(
    id      UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    name    TEXT NOT NULL,
    content JSON NOT NULL
);

CREATE TABLE IF NOT EXISTS history
(
    id          UUID PRIMARY KEY         NOT NULL DEFAULT gen_random_uuid(),
    pipeline_id UUID                     NOT NULL,
    status      TEXT                     NOT NULL,
    start_at    TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    end_at      TIMESTAMP WITH TIME ZONE,
    error       TEXT,
    result      JSON
);

CREATE INDEX IF NOT EXISTS idx_history_pipeline_id ON history (pipeline_id);

CREATE TABLE IF NOT EXISTS routes
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    pipeline_id UUID             NOT NULL,
    path        TEXT             NOT NULL,
    method      TEXT             NOT NULL
);

CREATE TABLE IF NOT EXISTS cron_jobs
(
    id          UUID PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    pipeline_id UUID             NOT NULL,
    expr        TEXT             NOT NULL,
    input_data  JSON             NOT NULL
);

COMMIT;
