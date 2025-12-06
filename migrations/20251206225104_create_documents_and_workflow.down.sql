-- Drop tables
DROP TABLE IF EXISTS workflow_templates;
DROP TABLE IF EXISTS projects;
DROP TABLE IF EXISTS workflow_tasks;
DROP TABLE IF EXISTS document_versions;
DROP TABLE IF EXISTS documents;

-- Drop enum types
DROP TYPE IF EXISTS stage_status;
DROP TYPE IF EXISTS task_status;
