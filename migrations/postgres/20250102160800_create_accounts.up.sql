create extension if not exists citext;

create table accounts (
    id uuid primary key,
    username citext not null unique
);
