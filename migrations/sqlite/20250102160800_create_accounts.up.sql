create table accounts (
    id text primary key not null,
    username text not null unique collate nocase
) strict;
