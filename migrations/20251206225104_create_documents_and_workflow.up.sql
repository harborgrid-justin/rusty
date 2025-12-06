-- Create enum types
CREATE TYPE task_status AS ENUM (
    'Pending',
    'In Progress',
    'Review',
    'Done',
    'Completed'
);

CREATE TYPE stage_status AS ENUM (
    'Pending',
    'Active',
    'Completed'
);

-- Create documents table
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    type VARCHAR(100) NOT NULL,
    content TEXT,
    upload_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_modified TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    tags TEXT[],
    file_size VARCHAR(50),
    source_module VARCHAR(100),
    status VARCHAR(50),
    is_encrypted BOOLEAN DEFAULT false,
    folder_id UUID,
    summary TEXT,
    risk_score INTEGER,
    linked_rules TEXT[],
    shared_with UUID[],
    is_redacted BOOLEAN DEFAULT false,
    author_id UUID REFERENCES users(id),
    form_fields JSONB,
    signing_status JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_documents_case_id ON documents(case_id);
CREATE INDEX idx_documents_type ON documents(type);
CREATE INDEX idx_documents_author_id ON documents(author_id);
CREATE INDEX idx_documents_upload_date ON documents(upload_date);

-- Create document versions table
CREATE TABLE document_versions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    version_number INTEGER NOT NULL,
    uploaded_by VARCHAR(255) NOT NULL,
    upload_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    content_snapshot TEXT,
    storage_key VARCHAR(500),
    author VARCHAR(255),
    author_id UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_document_versions_document_id ON document_versions(document_id);
CREATE INDEX idx_document_versions_version_number ON document_versions(version_number);

-- Create workflow tasks table
CREATE TABLE workflow_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(500) NOT NULL,
    status task_status NOT NULL DEFAULT 'Pending',
    assignee VARCHAR(255) NOT NULL,
    assignee_id UUID REFERENCES users(id),
    start_date TIMESTAMP WITH TIME ZONE,
    due_date TIMESTAMP WITH TIME ZONE NOT NULL,
    priority VARCHAR(20) NOT NULL CHECK (priority IN ('Low', 'Medium', 'High', 'Critical')),
    description TEXT,
    case_id UUID REFERENCES cases(id) ON DELETE CASCADE,
    project_id UUID,
    related_module VARCHAR(100),
    related_item_id VARCHAR(255),
    related_item_title VARCHAR(500),
    action_label VARCHAR(255),
    automated_trigger VARCHAR(255),
    linked_rules TEXT[],
    dependencies UUID[],
    completion INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_workflow_tasks_case_id ON workflow_tasks(case_id);
CREATE INDEX idx_workflow_tasks_assignee_id ON workflow_tasks(assignee_id);
CREATE INDEX idx_workflow_tasks_status ON workflow_tasks(status);
CREATE INDEX idx_workflow_tasks_due_date ON workflow_tasks(due_date);
CREATE INDEX idx_workflow_tasks_priority ON workflow_tasks(priority);

-- Create projects table
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL,
    priority VARCHAR(20) NOT NULL,
    lead VARCHAR(255) NOT NULL,
    due_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_projects_case_id ON projects(case_id);
CREATE INDEX idx_projects_status ON projects(status);

-- Create workflow templates table
CREATE TABLE workflow_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(500) NOT NULL,
    category VARCHAR(100) NOT NULL,
    complexity VARCHAR(20) NOT NULL CHECK (complexity IN ('Low', 'Medium', 'High')),
    duration VARCHAR(100),
    tags TEXT[],
    audit_ready BOOLEAN DEFAULT false,
    stages JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_workflow_templates_category ON workflow_templates(category);
CREATE INDEX idx_workflow_templates_complexity ON workflow_templates(complexity);
