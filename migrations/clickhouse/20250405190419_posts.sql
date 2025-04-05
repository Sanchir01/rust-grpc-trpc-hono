-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS posts (
  chat_id Int64,
  message_id UInt32,
  timestamp DateTime,
  post_timestamp DateTime,
  text String
) ENGINE = MergeTree()
ORDER BY (chat_id, message_id);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
SELECT 'down SQL query';
-- +goose StatementEnd
