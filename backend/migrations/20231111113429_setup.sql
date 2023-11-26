create extension if not exists "uuid-ossp";
create extension if not exists "pgcrypto";
create extension if not exists "pg_trgm";


create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format(
        'CREATE TRIGGER set_updated_at
            BEFORE UPDATE
            ON %s
            FOR EACH ROW
            WHEN (OLD is distinct from NEW)
        EXECUTE FUNCTION set_updated_at();', 
        tablename
    );
end;
$$ language plpgsql;

create or replace function nanoid(size int default 16)
    returns text as 
$$
declare
  id text := '';
  i int := 0;
  alphabet char(64) := 'abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz0123456789_-';
  bytes bytea := gen_random_bytes(size);
  byte int;
  pos int;
begin
    while i < size loop
        byte := get_byte(bytes, i);
        pos := (byte & 63) + 1;
        id := id || substr(alphabet, pos, 1);
        i = i + 1;
    end loop;
  return id;
end
$$ language plpgsql stable;

create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
