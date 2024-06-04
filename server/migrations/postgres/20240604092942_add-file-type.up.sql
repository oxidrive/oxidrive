create type file_type as enum ('file', 'folder');
alter table files add column type file_type not null default 'file';
