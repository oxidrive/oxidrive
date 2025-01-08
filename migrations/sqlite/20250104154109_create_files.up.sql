create table files (
    id text primary key not null,
    owner_id text not null,
    name text not null,
    content_type text not null,
    size integer not null default 0,
    tags text not null default '{}',
    unique(owner_id, name),
    foreign key (owner_id) references accounts(id)
) strict;
