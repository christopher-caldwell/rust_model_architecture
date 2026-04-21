-- ============================================================================
-- 02_bootstrap_object_privileges.sql
-- Run as: library_migrate
-- Connected to: library
-- Purpose: grant access to existing objects and set default privileges for
--          future objects created by library_admin_role
-- ============================================================================

SET ROLE library_admin_role;

-- ----------------------------------------------------------------------------
-- Existing tables
-- ----------------------------------------------------------------------------
GRANT SELECT ON ALL TABLES IN SCHEMA library TO
      library_maintenance_role
    , library_ro_role
    , library_rw_role
;

-- GRANT SELECT ON ALL TABLES IN SCHEMA etl TO
--       library_maintenance_role
--     , library_ro_role
--     , library_rw_role
-- ;

GRANT INSERT, UPDATE ON ALL TABLES IN SCHEMA library TO
      library_maintenance_role
    , library_rw_role
;

-- GRANT INSERT, UPDATE ON ALL TABLES IN SCHEMA etl TO
--       library_maintenance_role
--     , library_rw_role
-- ;

GRANT DELETE, TRUNCATE ON ALL TABLES IN SCHEMA library TO
      library_maintenance_role
;

-- GRANT DELETE, TRUNCATE ON ALL TABLES IN SCHEMA etl TO
--       library_maintenance_role
-- ;

-- ----------------------------------------------------------------------------
-- Existing sequences
-- ----------------------------------------------------------------------------
GRANT USAGE, SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA library TO
      library_maintenance_role
    , library_rw_role
;

-- GRANT USAGE, SELECT, UPDATE ON ALL SEQUENCES IN SCHEMA etl TO
--       library_maintenance_role
--     , library_rw_role
-- ;

-- ----------------------------------------------------------------------------
-- Default privileges for future tables created by library_admin_role
-- ----------------------------------------------------------------------------
ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA library
GRANT SELECT ON TABLES TO
      library_maintenance_role
    , library_ro_role
    , library_rw_role
;

-- ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA etl
-- GRANT SELECT ON TABLES TO
--       library_maintenance_role
--     , library_ro_role
--     , library_rw_role
-- ;

ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA library
GRANT INSERT, UPDATE ON TABLES TO
      library_maintenance_role
    , library_rw_role
;

-- ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA etl
-- GRANT INSERT, UPDATE ON TABLES TO
--       library_maintenance_role
--     , library_rw_role
-- ;

ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA library
GRANT DELETE, TRUNCATE ON TABLES TO
      library_maintenance_role
;

-- ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA etl
-- GRANT DELETE, TRUNCATE ON TABLES TO
--       library_maintenance_role
-- ;

-- ----------------------------------------------------------------------------
-- Default privileges for future sequences created by library_admin_role
-- ----------------------------------------------------------------------------
ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA library
GRANT USAGE, SELECT, UPDATE ON SEQUENCES TO
      library_maintenance_role
    , library_rw_role
;

-- ALTER DEFAULT PRIVILEGES FOR ROLE library_admin_role IN SCHEMA etl
-- GRANT USAGE, SELECT, UPDATE ON SEQUENCES TO
--       library_maintenance_role
--     , library_rw_role
-- ;

RESET ROLE;
