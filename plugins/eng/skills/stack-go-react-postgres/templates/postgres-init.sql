-- <project>/local/postgres-init/01-roles.sql
--
-- Mounted into the Postgres container at /docker-entrypoint-initdb.d/. The image runs
-- everything in that directory ONCE, as the superuser, on an empty data directory only —
-- so this never re-runs, and `docker compose down -v` is what replays it.
--
-- ⚠ Roles are CLUSTER-level, not schema-level. They are not created by a migration in
--   migration/, because a migration runs inside one database and these outlive it. This is
--   the one piece of SQL that is infrastructure rather than schema.
--
-- ⚠ This is the LOCAL bootstrap. A managed/deployed database creates its roles once, by
--   whatever mechanism that platform provides, and this file is not it — the passwords here
--   are unusable anywhere else by construction.
--
-- How many roles beyond these two, and which grants each holds, is a project decision — the
-- schema design lane owns it. Two is the floor, not the recommendation.

-- The role that owns the schema and runs migrations. Never the application's.
CREATE ROLE app_owner LOGIN PASSWORD 'local';

-- The role the application connects as. No CREATEDB, no CREATEROLE, no SUPERUSER,
-- and — the part that matters — NOT the owner of any table.
CREATE ROLE app_user LOGIN PASSWORD 'local';

-- The owner needs the database to build the schema in.
ALTER DATABASE app OWNER TO app_owner;

GRANT CONNECT ON DATABASE app TO app_user;
GRANT USAGE   ON SCHEMA public TO app_user;

-- ⚠ Without this, every table a future migration creates arrives with no grant for
--   app_user, and the application gets `permission denied` on a table that exists.
--   ALTER DEFAULT PRIVILEGES only applies to objects created AFTER it, and only to those
--   created by the role it names — which is why it is here and not at the end of a migration.
ALTER DEFAULT PRIVILEGES FOR ROLE app_owner IN SCHEMA public
  GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_user;
ALTER DEFAULT PRIVILEGES FOR ROLE app_owner IN SCHEMA public
  GRANT USAGE, SELECT ON SEQUENCES TO app_user;

-- ⚠ Owning a table bypasses that table's RLS policies unless the table is set to FORCE.
--   app_user owns nothing here, which is what keeps its policies live. If a project ever
--   makes the application role an owner, FORCE ROW LEVEL SECURITY becomes mandatory and
--   is easy to forget, because everything keeps working.
