-- Drop tables
DROP TABLE IF EXISTS trial_exhibits;
DROP TABLE IF EXISTS chain_of_custody_events;
DROP TABLE IF EXISTS evidence_items;
DROP TABLE IF EXISTS docket_entries;
DROP TABLE IF EXISTS motions;

-- Drop enum types
DROP TYPE IF EXISTS admissibility_status;
DROP TYPE IF EXISTS evidence_type;
DROP TYPE IF EXISTS docket_entry_type;
DROP TYPE IF EXISTS motion_outcome;
DROP TYPE IF EXISTS motion_status;
DROP TYPE IF EXISTS motion_type;
