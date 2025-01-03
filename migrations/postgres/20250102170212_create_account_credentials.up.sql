create table account_credentials (
    id text not null,
    account_id uuid not null references accounts(id),
    kind text not null,
    data jsonb not null,
    primary key (account_id, id)
);
