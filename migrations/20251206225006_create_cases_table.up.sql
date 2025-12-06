-- Create enum types
CREATE TYPE case_status AS ENUM (
    'Pre-Filing',
    'Discovery',
    'Trial',
    'Settled',
    'Closed',
    'Appeal',
    'Transferred'
);

CREATE TYPE matter_type AS ENUM (
    'Litigation',
    'M&A',
    'IP',
    'Real Estate',
    'General',
    'Appeal'
);

CREATE TYPE billing_model AS ENUM (
    'Hourly',
    'Fixed',
    'Contingency',
    'Hybrid'
);

-- Create cases table
CREATE TABLE cases (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(500) NOT NULL,
    client VARCHAR(255) NOT NULL,
    client_id UUID,
    matter_type matter_type NOT NULL,
    matter_sub_type VARCHAR(100),
    status case_status NOT NULL DEFAULT 'Pre-Filing',
    filing_date TIMESTAMP WITH TIME ZONE NOT NULL,
    description TEXT,
    value DECIMAL(15, 2),
    jurisdiction VARCHAR(200),
    court VARCHAR(200),
    judge VARCHAR(200),
    magistrate_judge VARCHAR(200),
    opposing_counsel VARCHAR(255),
    orig_case_number VARCHAR(100),
    orig_court VARCHAR(200),
    orig_judgment_date TIMESTAMP WITH TIME ZONE,
    notice_of_appeal_date TIMESTAMP WITH TIME ZONE,
    owner_id UUID REFERENCES users(id),
    owner_org_id UUID,
    lead_case_id UUID REFERENCES cases(id),
    is_consolidated BOOLEAN DEFAULT false,
    date_terminated TIMESTAMP WITH TIME ZONE,
    nature_of_suit VARCHAR(255),
    billing_model billing_model,
    pacer_data JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Create indexes
CREATE INDEX idx_cases_status ON cases(status);
CREATE INDEX idx_cases_client_id ON cases(client_id);
CREATE INDEX idx_cases_owner_id ON cases(owner_id);
CREATE INDEX idx_cases_filing_date ON cases(filing_date);
CREATE INDEX idx_cases_deleted_at ON cases(deleted_at);
CREATE INDEX idx_cases_matter_type ON cases(matter_type);

-- Create parties table
CREATE TABLE parties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    role VARCHAR(100) NOT NULL,
    type VARCHAR(50) NOT NULL CHECK (type IN ('Individual', 'Corporation', 'Government')),
    contact VARCHAR(255),
    counsel VARCHAR(255),
    party_group VARCHAR(100),
    linked_org_id UUID,
    address TEXT,
    phone VARCHAR(50),
    email VARCHAR(255),
    representation_type VARCHAR(100),
    attorneys JSONB,
    pacer_data JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_parties_case_id ON parties(case_id);
CREATE INDEX idx_parties_name ON parties(name);
CREATE INDEX idx_parties_role ON parties(role);

-- Create linked cases table (for case associations)
CREATE TABLE case_associations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    linked_case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    relationship_type VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(case_id, linked_case_id)
);

CREATE INDEX idx_case_associations_case_id ON case_associations(case_id);
CREATE INDEX idx_case_associations_linked_case_id ON case_associations(linked_case_id);
