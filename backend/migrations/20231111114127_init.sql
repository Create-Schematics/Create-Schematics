create table users
(
    user_id        uuid        primary key default uuid_generate_v1mc(),
    username       text        not null    unique collate "case_insensitive",
    displayname    text        not null,
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

create table punishments
(
    punishment_id uuid        primary key default uuid_generate_v1mc(),
    user_id       uuid        not null    references users (user_id) on delete cascade,
    issuer_id     uuid        not null    references users (user_id) on delete cascade,
    -- If the duration is null we assume that it is a permanent ban, while we could remove
    -- the user entirely, in the current model this would also clear all knowledge of their
    -- oauth providers allowing them to re-open the account, this also prevents confusion
    -- from duplicate usernames
    reason        text,
    until         timestamptz             default now(),
    created_at    timestamptz not null    default now()
);

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

create table mods
(
    mod_id          uuid         primary key default uuid_generate_v1mc(),
    mod_slug        text         not null    unique,
    mod_name        text,
    curseforge_slug int                      unique,
    modrinth_slug   text                     unique,
    created_at      timestamptz  not null    default now(),
    updated_at      timestamptz
);

select trigger_updated_at('mods');

create table mod_proposals
(
    proposal_id     uuid         primary key default uuid_generate_v1mc(),
    user_id         uuid         not null    references users (user_id)    on delete cascade,
    mod_id          uuid         not null    references mods (mod_id)      on delete cascade,
    mod_slug        text,
    mod_name        text,
    curseforge_slug int,
    modrinth_slug   text,
    created_at      timestamptz  not null    default now(),
    unique (user_id, mod_id)
);

create table mod_dependencies
(
    schematic_id uuid not null references schematics (schematic_id) on delete cascade,
    mod_id       uuid not null references mods  (mod_id)  on delete cascade,
    primary key (schematic_id, mod_id)
);

create table reports
(
    report_id    uuid        primary key default uuid_generate_v1mc(),
    user_id      uuid        not null    references users (user_id)           on delete cascade,
    schematic_id uuid        not null    references schematics (schematic_id) on delete cascade,
    body         text,
    created_at   timestamptz not null    default now(),
    unique (user_id, schematic_id)
);

create table comments
(
    comment_id     uuid        primary key  default uuid_generate_v1mc(),
    comment_author uuid        not null     references users (user_id)           on delete cascade,
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