create table files (
    id text primary key,
    name text not null,
    path citext unique not null,
    size bigint not null,
    user_id uuid,
    constraint fk_user_id foreign key(user_id) references users(id) on delete no action
);
