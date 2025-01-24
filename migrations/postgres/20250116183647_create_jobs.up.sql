create table jobs (
    id uuid primary key,
    kind text not null,
    body jsonb not null
);

create index idx_jobs_kind on jobs(kind);
