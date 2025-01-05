create table tags (
    key citext not null,
    value citext,
    file_id uuid not null,
    unique nulls not distinct (key, value, file_id)
);

create index idx_tags_file on tags (file_id);
