create table tags
(
    tag_id     bigserial    primary key,
    tag_name   text         not null     unique collate "case_insensitive",
    created_at timestamptz  not null     default now(),
    updated_at timestamptz
);

select trigger_updated_at('tags');

create table applied_tags
(
    tag_id       bigserial   not null references tags       (tag_id)       on delete cascade,
    schematic_id uuid        not null references schematics (schematic_id) on delete cascade,
    created_at   timestamptz not null default now(),
    primary key  (tag_id, schematic_id)
);

insert into tags 
    (tag_name)
values
    ('Steam Train'),
    ('Deisel Train'),
    ('Electric Train'),
    ('Crop Farm'),
    ('Mob Farm'),
    ('Windmill'),
    ('Steam Engine');