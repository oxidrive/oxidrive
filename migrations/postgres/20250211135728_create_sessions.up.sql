create table sessions(
    id uuid primary key,
    account_id uuid not null references accounts(id),
    expires_at timestamptz not null
);

create index idx_sessions_expiration on sessions (expires_at);
