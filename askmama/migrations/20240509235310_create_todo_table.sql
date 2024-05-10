CREATE TABLE todos (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL,
    created_at timestamptz NOT NULL
);
