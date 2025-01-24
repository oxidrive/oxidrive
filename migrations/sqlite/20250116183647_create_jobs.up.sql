create table jobs (
    id text primary key not null,
    kind text not null,
    body text not null
) strict;

create index idx_jobs_kind on jobs(kind);
