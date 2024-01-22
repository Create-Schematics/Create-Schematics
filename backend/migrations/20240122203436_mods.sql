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