create extension if not exists hstore;

create table files (
    id uuid primary key,
    owner_id uuid not null references accounts(id),
    name text not null,
    content_type text not null,
    size bigint not null default 0,
    tags hstore not null default '',
    unique(owner_id, name)
);

create index idx_files_tags on files using gin (tags);
