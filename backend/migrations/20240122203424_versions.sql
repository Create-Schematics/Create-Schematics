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

insert into create_versions 
    (create_version_name)
values
    ('0.2.4'),
    ('0.3'),
    ('0.3.1'),
    ('0.3.2'),
    ('0.4.1'),
    ('0.4'),
    ('0.5'),
    ('0.5.1');

insert into game_versions
    (game_version_name)
values
    ('1.16.5'),
    ('1.18'),
    ('1.18.1'),
    ('1.18.2'),
    ('1.19.2'),
    ('1.20.1');