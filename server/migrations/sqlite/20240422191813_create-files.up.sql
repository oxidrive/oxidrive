create table files (
    id text primary key,
    name text not null,
    path text unique not null,
    size bigint not null
);
