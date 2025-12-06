-- Drop tables
DROP TABLE IF EXISTS audit_logs;
DROP TABLE IF EXISTS conflict_checks;
DROP TABLE IF EXISTS risks;
DROP TABLE IF EXISTS invoices;
DROP TABLE IF EXISTS time_entries;
DROP TABLE IF EXISTS clients;
DROP TABLE IF EXISTS production_sets;
DROP TABLE IF EXISTS esi_sources;
DROP TABLE IF EXISTS depositions;
DROP TABLE IF EXISTS discovery_requests;

-- Drop enum types
DROP TYPE IF EXISTS risk_status;
DROP TYPE IF EXISTS risk_level;
DROP TYPE IF EXISTS risk_category;
DROP TYPE IF EXISTS discovery_status;
DROP TYPE IF EXISTS discovery_type;
