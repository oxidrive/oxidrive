create extension if not exists citext; -- case insensitive text

create table users (
    id uuid primary key,
    username citext unique not null,
    password_hash text not null
);
