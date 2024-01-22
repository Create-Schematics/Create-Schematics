create table users
(
    user_id        uuid        primary key default uuid_generate_v1mc(),
    -- The username must be unique to enusre it can be used to identify the user in urls, 
    -- without needing to use their id this is collected from the oauth provider, if it is
    -- already taken then we append a set of random digits to the end and then check if that
    -- is unique until a new username is discovered, if wanted this can then be replaced by 
    -- the user at a later date
    --
    -- i.e If the username 'rabbitminers' was taken a three digit number will be appended
    -- to the end such as 'rabbitminers123'
    --
    -- Note because of the case insensitive collation usernames cannot be searched with `LIKE`
    -- or `ILIKE` operators, if this is needed in the future add a sperate index and remove this
    -- comment
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
    -- This is the user id of the moderator who issued the punishment
    issuer_id     uuid        not null    references users (user_id) on delete cascade,
    -- If the duration is null we assume that it is a permanent ban, while we could remove
    -- the user entirely, in the current model this would also clear all knowledge of their
    -- oauth providers allowing them to re-open the account, this also prevents confusion
    -- from duplicate usernames
    reason        text,
    until         timestamptz             default now(),
    created_at    timestamptz not null    default now()
);