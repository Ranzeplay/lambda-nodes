BEGIN;

INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
    'BeginRequest',
    TRUE,
    'internal',
    '{}',
    ARRAY['data', 'context']
);

INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
    'EndRequest',
    TRUE,
    'internal',
    ARRAY['data', 'context'],
    '{}'
);

INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
           'True',
           TRUE,
           'internal',
           '{}',
           ARRAY['out']
       );

INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
           'False',
           TRUE,
           'internal',
           '{}',
           ARRAY['out']
       );

INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
           'Breaker',
           TRUE,
           'internal',
           ARRAY['condition'],
           '{}'
       );

INSERT INTO nodes (name, is_internal, script, inputs, outputs)
VALUES (
           'Empty',
           TRUE,
           'internal',
           '{}',
           ARRAY['out']
       );

COMMIT;
