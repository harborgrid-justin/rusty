-- Create enum types
CREATE TYPE motion_type AS ENUM (
    'Dismiss',
    'Summary Judgment',
    'Compel Discovery',
    'In Limine',
    'Continuance',
    'Sanctions'
);

CREATE TYPE motion_status AS ENUM (
    'Draft',
    'Filed',
    'Opposition Served',
    'Reply Served',
    'Hearing Set',
    'Submitted',
    'Decided',
    'Withdrawn'
);

CREATE TYPE motion_outcome AS ENUM (
    'Granted',
    'Denied',
    'Withdrawn',
    'Moot'
);

CREATE TYPE docket_entry_type AS ENUM (
    'Filing',
    'Order',
    'Notice',
    'Minute Entry',
    'Exhibit',
    'Hearing'
);

CREATE TYPE evidence_type AS ENUM (
    'Physical',
    'Digital',
    'Document',
    'Testimony',
    'Forensic'
);

CREATE TYPE admissibility_status AS ENUM (
    'Admissible',
    'Challenged',
    'Inadmissible',
    'Pending'
);

-- Create motions table
CREATE TABLE motions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    type motion_type NOT NULL,
    status motion_status NOT NULL DEFAULT 'Draft',
    outcome motion_outcome,
    filing_date TIMESTAMP WITH TIME ZONE,
    hearing_date TIMESTAMP WITH TIME ZONE,
    opposition_due_date TIMESTAMP WITH TIME ZONE,
    reply_due_date TIMESTAMP WITH TIME ZONE,
    documents TEXT[],
    assigned_attorney VARCHAR(255),
    linked_rules TEXT[],
    conferral_status VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_motions_case_id ON motions(case_id);
CREATE INDEX idx_motions_status ON motions(status);
CREATE INDEX idx_motions_type ON motions(type);
CREATE INDEX idx_motions_filing_date ON motions(filing_date);

-- Create docket entries table
CREATE TABLE docket_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sequence_number INTEGER NOT NULL,
    pacer_sequence_number INTEGER,
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    type docket_entry_type NOT NULL,
    title VARCHAR(500) NOT NULL,
    description TEXT,
    filed_by VARCHAR(255),
    is_sealed BOOLEAN DEFAULT false,
    document_id UUID REFERENCES documents(id),
    structured_data JSONB,
    triggers_deadlines JSONB,
    doc_link VARCHAR(500),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_docket_entries_case_id ON docket_entries(case_id);
CREATE INDEX idx_docket_entries_date ON docket_entries(date);
CREATE INDEX idx_docket_entries_type ON docket_entries(type);
CREATE INDEX idx_docket_entries_sequence_number ON docket_entries(sequence_number);

-- Create evidence items table
CREATE TABLE evidence_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    title VARCHAR(500) NOT NULL,
    type evidence_type NOT NULL,
    description TEXT NOT NULL,
    collection_date TIMESTAMP WITH TIME ZONE NOT NULL,
    collected_by VARCHAR(255) NOT NULL,
    custodian VARCHAR(255) NOT NULL,
    location VARCHAR(500) NOT NULL,
    admissibility admissibility_status NOT NULL DEFAULT 'Pending',
    tags TEXT[],
    blockchain_hash VARCHAR(255),
    tracking_uuid UUID NOT NULL DEFAULT gen_random_uuid(),
    chunks JSONB,
    file_size VARCHAR(50),
    file_type VARCHAR(100),
    linked_rules TEXT[],
    status VARCHAR(100),
    authentication_method VARCHAR(100),
    hearsay_status VARCHAR(100),
    is_original BOOLEAN,
    relevance_score INTEGER,
    expert_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_evidence_items_case_id ON evidence_items(case_id);
CREATE INDEX idx_evidence_items_type ON evidence_items(type);
CREATE INDEX idx_evidence_items_admissibility ON evidence_items(admissibility);
CREATE INDEX idx_evidence_items_tracking_uuid ON evidence_items(tracking_uuid);

-- Create chain of custody events table
CREATE TABLE chain_of_custody_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    evidence_id UUID NOT NULL REFERENCES evidence_items(id) ON DELETE CASCADE,
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    action VARCHAR(255) NOT NULL,
    actor VARCHAR(255) NOT NULL,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_chain_of_custody_events_evidence_id ON chain_of_custody_events(evidence_id);
CREATE INDEX idx_chain_of_custody_events_date ON chain_of_custody_events(date);

-- Create trial exhibits table
CREATE TABLE trial_exhibits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    case_id UUID NOT NULL REFERENCES cases(id) ON DELETE CASCADE,
    exhibit_number VARCHAR(50) NOT NULL,
    title VARCHAR(500) NOT NULL,
    date_marked TIMESTAMP WITH TIME ZONE NOT NULL,
    party VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL,
    file_type VARCHAR(100) NOT NULL,
    description TEXT,
    witness VARCHAR(255),
    uploaded_by VARCHAR(255),
    tags TEXT[],
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    version INTEGER DEFAULT 1,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_trial_exhibits_case_id ON trial_exhibits(case_id);
CREATE INDEX idx_trial_exhibits_exhibit_number ON trial_exhibits(exhibit_number);
CREATE INDEX idx_trial_exhibits_party ON trial_exhibits(party);
