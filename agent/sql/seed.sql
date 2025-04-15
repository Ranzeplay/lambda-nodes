BEGIN;

-- Seed the nodes table with an OnRequest node
INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
    'BeginRequest',
    TRUE,
    'internal',
    '{}',
    ARRAY['data', 'context']
);

-- Seed the nodes table with an EndRequest node
INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
    'EndRequest',
    TRUE,
    'internal',
    ARRAY['data', 'context'],
    '{}'
);

COMMIT;
