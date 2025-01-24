create table collections (
    id uuid primary key,
    owner_id uuid not null references accounts(id),
    name text not null,
    filter text not null
);

create index idx_collections_owned_by on collections (owner_id);

create table collections_files (
    collection_id uuid not null references collections(id),
    file_id uuid not null references files(id),
    primary key (collection_id, file_id)
);

create index idx_collections_files on collections_files (collection_id);
