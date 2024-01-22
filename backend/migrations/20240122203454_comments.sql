create table comments
(
    comment_id     uuid        primary key  default uuid_generate_v1mc(),
    parent         uuid                     references comments (comment_id)     on delete cascade,
    comment_author uuid        not null     references users (user_id)           on delete cascade,
    comment_body   text        not null,
    schematic_id   uuid        not null     references schematics (schematic_id) on delete cascade,
    created_at     timestamptz not null     default now(),
    updated_at     timestamptz
);

select trigger_updated_at('comments');

create index on comments (schematic_id, created_at);