-- +goose Up
-- +goose StatementBegin
CREATE TABLE todos (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    title TEXT NOT NULL,
    completed BOOLEAN NOT NULL,
    created_at timestamptz NOT NULL
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE todos;
-- +goose StatementEnd
