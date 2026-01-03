CREATE TYPE business_model_type AS ENUM (
    'contract', -- 請負
    'ses',      -- 準委任
    'saas',     -- Saas
    'media',    -- メディア
    'internal', -- 社内業務
    'rnd'       -- 研究開発
);

CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    business_model business_model_type NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_jobs_project_id ON jobs(project_id);