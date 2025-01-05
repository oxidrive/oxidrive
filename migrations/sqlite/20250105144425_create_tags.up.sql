create table tags (
    key text not null,
    value text,
    file_id text not null
) strict;

create unique index idx_tags_key_value_file on tags (key, ifnull(value, 0), file_id);
create index idx_tags_file on tags (file_id);
