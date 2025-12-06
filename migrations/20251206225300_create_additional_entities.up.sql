-- Create enum types
CREATE TYPE organization_type AS ENUM (
    'LawFirm',
    'Corporate',
    'Government',
    'Court',
    'Vendor'
);

CREATE TYPE legal_rule_type AS ENUM (
    'FRE',
    'FRCP',
    'FRAP',
    'Local',
    'State'
);

CREATE TYPE entity_type AS ENUM (
    'Individual',
    'Corporation',
    'Court',
    'Government',
    'Vendor',
    'Law Firm'
);

-- Create organizations table
CREATE TABLE organizations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    type organization_type NOT NULL,
    domain VARCHAR(255) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_organizations_type ON organizations(type);
CREATE INDEX idx_organizations_status ON organizations(status);

-- Create groups table
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    org_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    permissions TEXT[],
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_groups_org_id ON groups(org_id);

-- Create citations table
CREATE TABLE citations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    citation VARCHAR(500) NOT NULL,
    title VARCHAR(500) NOT NULL,
    type VARCHAR(100) NOT NULL,
    description TEXT,
    relevance VARCHAR(50) NOT NULL,
    shepards_signal VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_citations_citation ON citations(citation);
CREATE INDEX idx_citations_type ON citations(type);

-- Create legal rules table
CREATE TABLE legal_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(100) NOT NULL,
    name VARCHAR(500) NOT NULL,
    type legal_rule_type NOT NULL,
    level VARCHAR(100),
    summary TEXT,
    text TEXT,
    parent_id UUID REFERENCES legal_rules(id),
    structured_content JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_legal_rules_code ON legal_rules(code);
CREATE INDEX idx_legal_rules_type ON legal_rules(type);
CREATE INDEX idx_legal_rules_parent_id ON legal_rules(parent_id);

-- Create legal entities table
CREATE TABLE legal_entities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    type entity_type NOT NULL,
    roles TEXT[],
    email VARCHAR(255),
    phone VARCHAR(50),
    website VARCHAR(255),
    address TEXT,
    city VARCHAR(100),
    state VARCHAR(50),
    country VARCHAR(100),
    tax_id VARCHAR(100),
    company VARCHAR(255),
    title VARCHAR(255),
    bar_number VARCHAR(100),
    jurisdiction VARCHAR(200),
    status VARCHAR(50) NOT NULL CHECK (status IN ('Active', 'Inactive', 'Prospect', 'Blacklisted', 'Deceased')),
    risk_score INTEGER DEFAULT 0,
    tags TEXT[],
    notes TEXT,
    linked_user_id UUID REFERENCES users(id),
    avatar VARCHAR(500),
    external_ids JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_legal_entities_name ON legal_entities(name);
CREATE INDEX idx_legal_entities_type ON legal_entities(type);
CREATE INDEX idx_legal_entities_status ON legal_entities(status);

-- Create entity relationships table
CREATE TABLE entity_relationships (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_id UUID NOT NULL REFERENCES legal_entities(id) ON DELETE CASCADE,
    target_id UUID NOT NULL REFERENCES legal_entities(id) ON DELETE CASCADE,
    type VARCHAR(100) NOT NULL CHECK (type IN ('Employment', 'Subsidiary', 'Counsel_For', 'Sued_By', 'Witness_For', 'Family', 'Conflict', 'Board_Member')),
    description TEXT,
    start_date TIMESTAMP WITH TIME ZONE,
    end_date TIMESTAMP WITH TIME ZONE,
    active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_entity_relationships_source_id ON entity_relationships(source_id);
CREATE INDEX idx_entity_relationships_target_id ON entity_relationships(target_id);
CREATE INDEX idx_entity_relationships_type ON entity_relationships(type);

-- Create communications table
CREATE TABLE communications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    subject VARCHAR(500) NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    type VARCHAR(100) NOT NULL,
    direction VARCHAR(50) NOT NULL,
    sender VARCHAR(255) NOT NULL,
    recipient VARCHAR(255) NOT NULL,
    preview TEXT,
    has_attachment BOOLEAN DEFAULT false,
    status VARCHAR(50) NOT NULL,
    is_privileged BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_communications_case_id ON communications(case_id);
CREATE INDEX idx_communications_user_id ON communications(user_id);
CREATE INDEX idx_communications_date ON communications(date);
CREATE INDEX idx_communications_type ON communications(type);

-- Create clauses table (for knowledge base)
CREATE TABLE clauses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    category VARCHAR(100) NOT NULL,
    content TEXT NOT NULL,
    version INTEGER DEFAULT 1,
    usage_count INTEGER DEFAULT 0,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL,
    risk_rating VARCHAR(50) NOT NULL,
    versions JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_clauses_category ON clauses(category);
CREATE INDEX idx_clauses_name ON clauses(name);

-- Create notifications table
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    read BOOLEAN DEFAULT false,
    type VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_read ON notifications(read);
CREATE INDEX idx_notifications_time ON notifications(time);
