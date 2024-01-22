create table notifications
(
    notification_id uuid        primary key default uuid_generate_v1mc(),
    user_id         uuid        not null    references users (user_id) on delete cascade,
    title           text        not null,
    body            text        not null,
    link            text,
    read            boolean     not null    default false,
    created_at      timestamptz not null    default now()
);
