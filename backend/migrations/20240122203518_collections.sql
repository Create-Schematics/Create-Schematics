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