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

create table reports
(
    report_id    uuid        primary key default uuid_generate_v1mc(),
    user_id      uuid        not null    references users (user_id)           on delete cascade,
    schematic_id uuid        not null    references schematics (schematic_id) on delete cascade,
    body         text,
    created_at   timestamptz not null    default now(),
    unique (user_id, schematic_id)
);

create table schematic_likes
(
    schematic_id uuid        not null references schematics (schematic_id) on delete cascade,
    user_id      uuid        not null references users      (user_id)      on delete cascade,
    positive     boolean     not null,
    created_at   timestamptz not null default now(),
    updated_at   timestamptz,
    primary key (schematic_id, user_id)
);

