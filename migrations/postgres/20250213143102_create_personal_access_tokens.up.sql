create table personal_access_tokens (
    id uuid primary key,
    token_hash bytea not null unique,
    account_id uuid not null references accounts(id),
    expires_at timestamptz
);
