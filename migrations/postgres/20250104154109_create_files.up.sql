create table files (
    id uuid primary key,
    owner_id uuid not null references accounts(id),
    name text not null,
    content_type text not null,
    unique(owner_id, name)
);
