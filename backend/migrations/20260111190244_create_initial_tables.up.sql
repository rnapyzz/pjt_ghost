-- Add up migration script here

-- Users
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employee_id VARCHAR(5) NOT NULL UNIQUE,
    username VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) DEFAULT 'general' NOT NULL,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,

    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Segments
CREATE TABLE segments  (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,

    ui_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Themes
CREATE TABLE themes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(200) NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    segment_id UUID REFERENCES segments(id),

    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Services
CREATE TABLE services (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    slug VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    owner_id UUID REFERENCES users(id),
    segment_id UUID NOT NULL REFERENCES segments(id),

    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Projects
CREATE TABLE projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    theme_id UUID REFERENCES themes(id),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb,

    type VARCHAR(100) NOT NULL DEFAULT 'Normal',
    target_market TEXT,
    value_prop TEXT,
    target_client TEXT,
    kpis TEXT,

    is_active BOOLEAN DEFAULT TRUE NOT NULL,
    owner_id UUID REFERENCES users(id),

    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Jobs
CREATE TABLE jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    service_id UUID NOT NULL REFERENCES services(id),

    project_id UUID REFERENCES projects(id),
    theme_id UUID REFERENCES themes(id),

    title VARCHAR(200) NOT NULL,
    description TEXT,
    status VARCHAR(200) DEFAULT 'Draft' NOT NULL,
    owner_id UUID REFERENCES users(id),

    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TYPE account_type AS ENUM (
    'Revenue',
    'CostOfGoodsSold',
    'SellingGeneralAdmin'
);

CREATE TABLE account_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    account_type account_type NOT NULL,
    display_order INTEGER NOT NULL DEFAULT 0,
    description TEXT,
    is_active BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TYPE scenario_type AS ENUM (
    'MasterPlan',
    'RevisedPlan',
    'InitialPlan',
    'ExecPlanAdjust',
    'JobPlan',
    'Actual'
);

CREATE TABLE pl_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID NOT NULL REFERENCES projects(id),
    job_id UUID REFERENCES jobs(id),
    account_item_id UUID NOT NULL REFERENCES account_items(id),
    scenario scenario_type NOT NULL,
    date DATE NOT NULL,
    amount DECIMAL(19, 4) NOT NULL,
    description TEXT,
    created_by UUID NOT NULL REFERENCES users(id),
    updated_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);


CREATE INDEX idx_pl_entries_project_scenario ON pl_entries(project_id, scenario);
CREATE INDEX idx_pl_entries_date ON pl_entries(date);


-- 初期データ
INSERT INTO account_items (name, account_type, description, display_order, is_active) VALUES
('売上高', 'Revenue', '営業収益', 10, true),
('売上原価', 'CostOfGoodsSold', '制作費やコンテンツ費や人件費などの売上として提供する役務の価値を直接構成するような費用', 20, true),
('販管費', 'SellingGeneralAdmin', '売上として提供する役務の価値を直接構成するものとはいえない費用', 30, true);
