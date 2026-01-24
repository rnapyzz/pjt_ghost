-- Add down migration script here
DROP TABLE IF EXISTS pl_entries;
DROP TABLE IF EXISTS account_items;
DROP TABLE IF EXISTS jobs;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS services;
DROP TABLE IF EXISTS themes;
DROP TABLE IF EXISTS segments;
DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS scenario_type;
DROP TYPE IF EXISTS account_type;