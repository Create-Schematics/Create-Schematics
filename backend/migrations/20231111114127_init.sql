create table users
(
    user_id        uuid        primary key default uuid_generate_v1mc(),
    username       text        not null,
    avatar         text,
    email          text                    unique collate "case_insensitive",
    oauth_provider text        not null,
    oauth_id       text        not null,
    about          text,       
    role           text        not null    default 'user',
    created_at     timestamptz not null    default now(),
    updated_at     timestamptz
);

select trigger_updated_at('users');

create table create_versions
(
    create_version_id   serial      primary key,
    create_version_name text        not null,
    created_at          timestamptz not null default now()
);

create table game_versions
(
    game_version_id   serial      primary key,
    game_version_name text        not null,
    created_at        timestamptz not null default now()
);

create table schematics
(
    schematic_id      uuid        primary key  default uuid_generate_v1mc(),
    schematic_name    text        not null,
    body              text        not null,
    game_version_id   serial      not null     references game_versions (game_version_id),
    create_version_id serial      not null     references create_versions (create_version_id),
    author            uuid        not null     references users (user_id),
    images            text[]      not null,
    files             text[]      not null,
    downloads         integer     not null     default 0,
    created_at        timestamptz not null     default now(),
    updated_at        timestamptz
);

select trigger_updated_at('schematics');

create table comments
(
    comment_id     uuid        primary key  default uuid_generate_v1mc(),
    comment_author uuid        not null     references users (user_id)           on delete cascade unique,
    comment_body   text        not null,
    schematic_id   uuid        not null     references schematics (schematic_id) on delete cascade,
    created_at     timestamptz not null     default now(),
    updated_at     timestamptz
);

select trigger_updated_at('comments');

create index on comments (schematic_id, created_at);

create table schematic_likes
(
    schematic_id uuid        not null references schematics (schematic_id) on delete cascade,
    user_id      uuid        not null references users      (user_id)      on delete cascade,
    positive     boolean     not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz,
    primary key (schematic_id, user_id)
);

select trigger_updated_at('schematic_likes');

create table mods
(
    mod_id          text primary key,
    mod_name        text unique collate "case_insensitive",
    curseforge_slug int  unique,
    modrinth_slug   text unique,
    created_at      timestamptz  not null default now(),
    updated_at      timestamptz
);

select trigger_updated_at('mods');

create table mod_dependencies
(
    schematic_id uuid not null references schematics (schematic_id) on delete cascade,
    mod_id       text not null references mods       (mod_id)       on delete cascade,
    -- Ensure one schematic cannot depend on the same mod twice
    primary key (schematic_id, mod_id)
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
    schematic_id uuid        not null references schematics (schematic_id) on delete cascade,
    created_at   timestamptz not null default now(),
    primary key (tag_id, schematic_id)
);

create table collections
(
    collection_id   uuid        primary key default uuid_generate_v1mc(),
    user_id         uuid        not null    references users (user_id)    on delete cascade,
    collection_name text        not null,
    is_private      boolean     not null,
    created_at      timestamptz not null    default now(),
    updated_at      timestamptz
);

select trigger_updated_at('collections');

create table collection_entries
(
    schematic_id  uuid        not null references schematics  (schematic_id)  on delete cascade,
    collection_id uuid        not null references collections (collection_id) on delete cascade,
    created_at    timestamptz not null default now(),
    primary key   (collection_id, schematic_id)
);