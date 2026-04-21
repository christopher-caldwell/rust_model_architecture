BEGIN;

-- ============================================================================
-- 00_bootstrap_database_and_roles.sql
-- Run as: postgres
-- Purpose: create database, roles, users, memberships, and connection defaults
-- ============================================================================

CREATE DATABASE library;
REVOKE ALL ON DATABASE library FROM PUBLIC;

-- ----------------------------------------------------------------------------
-- No-login roles
-- ----------------------------------------------------------------------------
CREATE ROLE library_admin_role NOLOGIN;
CREATE ROLE library_maintenance_role NOLOGIN;
CREATE ROLE library_ro_role NOLOGIN;
CREATE ROLE library_rw_role NOLOGIN;

-- ----------------------------------------------------------------------------
-- Login users
-- ----------------------------------------------------------------------------
CREATE USER library_admin       WITH LOGIN PASSWORD 'admin';
CREATE USER library_migrate     WITH LOGIN PASSWORD 'migrate';
CREATE USER library_maintenance WITH LOGIN PASSWORD 'maintenance';
CREATE USER library_ro          WITH LOGIN PASSWORD 'ro';
CREATE USER library_rw          WITH LOGIN PASSWORD 'rw';

-- ----------------------------------------------------------------------------
-- Memberships
-- ----------------------------------------------------------------------------
GRANT library_admin_role       TO library_admin;
GRANT library_admin_role       TO library_migrate;
GRANT library_maintenance_role TO library_maintenance;
GRANT library_ro_role          TO library_ro;
GRANT library_rw_role          TO library_rw;

-- ----------------------------------------------------------------------------
-- Database adminship and access
-- ----------------------------------------------------------------------------
ALTER DATABASE library OWNER TO library_admin_role;

GRANT CONNECT ON DATABASE library TO
      library_admin
    , library_migrate
    , library_maintenance
    , library_ro
    , library_rw
;

-- ----------------------------------------------------------------------------
-- Default search_path
-- etl first so tools that expect a default schema land there, but include
-- library so both schemas are visible by default.
-- ----------------------------------------------------------------------------
ALTER DATABASE library SET search_path TO library;

ALTER ROLE library_admin       IN DATABASE library SET search_path TO library;
ALTER ROLE library_migrate     IN DATABASE library SET search_path TO library;
ALTER ROLE library_maintenance IN DATABASE library SET search_path TO library;
ALTER ROLE library_ro          IN DATABASE library SET search_path TO library;
ALTER ROLE library_rw          IN DATABASE library SET search_path TO library;

COMMIT;
