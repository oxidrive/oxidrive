create table collections (
    id text not null primary key,
    owner_id text not null,
    name text not null,
    filter text not null,
    foreign key (owner_id) references accounts(id)
) strict;

create index idx_collections_owned_by on collections (owner_id);

create table collections_files (
    collection_id text not null,
    file_id text not null,
    foreign key (collection_id) references collections(id),
    foreign key (file_id) references files(id),
    primary key (collection_id, file_id)
) strict;

create index idx_collections_files on collections_files (collection_id);
