create table sessions(
    id text primary key not null,
    account_id text not null,
    expires_at text not null,
    foreign key (account_id) references accounts(id)
) strict;

create index idx_sessions_expiration on sessions (expires_at);
