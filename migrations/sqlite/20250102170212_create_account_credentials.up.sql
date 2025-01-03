create table account_credentials (
    id text not null,
    account_id text not null,
    kind text not null,
    data text not null,
    primary key (account_id, id),
    foreign key (account_id) references accounts(id)
) strict;
