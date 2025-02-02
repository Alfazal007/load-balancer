-- +goose Up
CREATE TABLE servers (
	id serial primary key,
    server_url text unique not null
);

-- +goose Down
DROP TABLE servers;

