BEGIN;

-- Drop tables first (in reverse order of creation to handle dependencies)
DROP TABLE IF EXISTS pipelines;
DROP TABLE IF EXISTS nodes;
DROP TABLE IF EXISTS logs;

-- Drop custom types
DROP TYPE IF EXISTS http_method;
DROP TYPE IF EXISTS log_level;

COMMIT;
