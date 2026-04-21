-- ============================================================================
-- 01_bootstrap_schemas.sql
-- Run as: library_admin
-- Connected to: library
-- Purpose: create schemas, set adminship, grant schema usage
-- ============================================================================

SET ROLE library_admin_role;

CREATE SCHEMA IF NOT EXISTS library AUTHORIZATION library_admin_role;
-- CREATE SCHEMA IF NOT EXISTS etl            AUTHORIZATION library_admin_role;

ALTER SCHEMA library OWNER TO library_admin_role;
-- ALTER SCHEMA etl            admin TO library_admin_role;

-- Optional hardening. Keep only if this is truly how you want to run the DB.
DROP SCHEMA IF EXISTS public CASCADE;

GRANT USAGE ON SCHEMA library TO
	library_maintenance_role
	, library_ro_role
	, library_rw_role
;

-- GRANT USAGE ON SCHEMA etl TO
--       library_maintenance_role
--     , library_ro_role
--     , library_rw_role
-- ;

RESET ROLE;
