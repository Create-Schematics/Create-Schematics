create extension if not exists "uuid-ossp";
create extension if not exists "pg_trgm";

-- Some tables have a `created_at` and `updated_at` column which aswell as being useful 
-- information on the frontend such as identifying when a schematic was uplaoded can be 
-- useful for auditing and debugging. `created_at` can be handled by setting a default
-- value for the column as `now()` updated_at would need to be set directly whenever the
-- table is updated or added indivdually as a trigger
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

-- Some enties such as 
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

-- This creates a text collation that allows text to be sorted case insensitively useful
-- for unique indexes on usernames and alike allowing us to ensure names are unique whithout
-- being constrained in case
create collation case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);
