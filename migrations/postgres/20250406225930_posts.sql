-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS post (
    message_id INTEGER PRIMARY KEY,
    chat_id BIGINT,
    timestamp TIMESTAMPTZ,
    post_timestamp TIMESTAMPTZ,
    text VARCHAR(500)
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
