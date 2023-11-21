create table users
(
    user_id       uuid        primary key default uuid_generate_v1mc(),
    username      text        not null    unique collate "case_insensitive",
    email         text        not null    unique collate "case_insensitive",
    password_hash text        not null,
    created_at    timestamptz not null    default now(),
    updated_at    timestamptz
);

select trigger_updated_at('users');

create table create_versions
(
    create_version_id serial primary key,
    version           text   not null     
);

create table game_versions
(
    game_version_id serial primary key,
    version         text   not null
);

create table schematics
(
    schematic_id   bigserial   primary key,
    schematic_name text        not null     collate "case_insensitive" unique,
    game_version   serial      not null     references game_versions,
    create_version serial      not null     references create_versions,
    author         uuid        not null     references users (user_id),
    downloads      integer     not null     default 0,
    created_at     timestamptz not null     default now(),
    updated_at     timestamptz
);

select trigger_updated_at('schematics');

create table comments
(
    comment_id     bigserial   primary key,
    comment_author uuid        not null     references users (user_id)           on delete cascade unique,
    comment_body   text        not null,
    schematic_id   bigserial   not null     references schematics (schematic_id) on delete cascade,
    created_at     timestamptz not null     default now(),
    updated_at     timestamptz
);

select trigger_updated_at('comments');

create index on comments (schematic_id, created_at);

create table schematic_likes
(
    schematic_id bigserial   not null references schematics (schematic_id) on delete cascade,
    user_id      uuid        not null references users      (user_id)      on delete cascade,
    positive     boolean     not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz,
    primary key (schematic_id, user_id)
);

select trigger_updated_at('schematic_likes');

create table mods
(
    mod_id          bigserial    primary key,
    -- Store the mods name to avoid needing to query modrinth and curseforge
    -- when searching for mods
    mod_name        text unique collate "case_insensitive",
    curseforge_slug int  unique,
    modrinth_slug   text unique,
    created_at      timestamptz  not null default now(),
    updated_at      timestamptz
);

select trigger_updated_at('mods');

create table mod_dependencies
(
    schematic_id bigserial not null references schematics (schematic_id) on delete cascade,
    mod_id       bigserial not null references mods       (mod_id)       on delete cascade,
    -- Ensure one schematic cannot depend on the same mod twice
    primary key (schematic_id, mod_id)
);

create table favorites
(
    schematic_id bigserial   not null references schematics (schematic_id) on delete cascade,
    user_id      uuid        not null references users      (user_id)      on delete cascade,
    created_at   timestamptz not null default now(),
    primary key (schematic_id, user_id)
);

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
    schematic_id bigserial   not null references schematics (schematic_id) on delete cascade,
    created_at   timestamptz not null default now(),
    primary key (tag_id, schematic_id)
);