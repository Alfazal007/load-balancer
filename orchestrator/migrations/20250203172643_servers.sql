CREATE TABLE servers (
    id serial primary key,
    server_url text unique not null
);
