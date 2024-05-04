create table tokens (
    id text primary key,
    user_id uuid not null,
    expires_at timestamp with time zone not null
);
