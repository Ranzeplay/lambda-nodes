BEGIN;

-- Drop tables first (in reverse order of creation to handle dependencies)
DROP TABLE IF EXISTS pipelines;
DROP TABLE IF EXISTS nodes;
DROP TABLE IF EXISTS logs;
DROP TABLE IF EXISTS history;
DROP TABLE IF EXISTS routes;
DROP TABLE IF EXISTS cron_jobs;

-- Drop custom types
DROP TYPE IF EXISTS log_level;

COMMIT;
