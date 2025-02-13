create table personal_access_tokens (
    id text not null primary key,
    token_hash blob not null unique,
    account_id text not null,
    expires_at text,
    foreign key (account_id) references accounts(id)
) strict;
