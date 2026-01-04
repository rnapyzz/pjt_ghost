-- 勘定科目の大分類の型定義
CREATE TYPE accounts_category AS ENUM (
    'sales',
    'cost_of_sales',
    'sga'
);

-- 勘定科目マスタ
CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    category accounts_category NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 項目種別マスタ
CREATE TABLE item_types (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    account_id UUID NOT NULL REFERENCES accounts(id),
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 項目テーブル
CREATE TABLE items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    job_id UUID NOT NULL REFERENCES jobs(id) ON DELETE CASCADE,
    item_type_id UUID NOT NULL REFERENCES item_types(id),
    assignee_id UUID REFERENCES users(id),

    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 計上データテーブル
CREATE TABLE entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    item_id UUID NOT NULL REFERENCES items(id) ON DELETE CASCADE,

    date DATE NOT NULL,
    amount BIGINT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (item_id, date)
);

-- インデックスの作成
CREATE INDEX idx_item_types_account_id ON item_types(account_id);
CREATE INDEX idx_items_job_id ON items(job_id);
CREATE INDEX idx_entries_item_id ON entries(item_id);

-- =============================================
-- 初期マスタデータの投入 (Seed Data) 
-- =============================================

-- ---------------------------------------------
-- A. 売上 (Sales)
-- ---------------------------------------------

WITH stock_acc AS (
    INSERT INTO accounts (name, category) VALUES ('ストック売上', 'sales') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY['SaaS利用料', '保守運用売上', 'サーバー管理代行費']) FROM stock_acc;

WITH flow_acc AS (
    INSERT INTO accounts (name, category) VALUES ('フロー売上', 'sales') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY['受託開発売上', 'SES売上', 'スポットコンサル費', '初期導入費']) FROM flow_acc;


-- ---------------------------------------------
-- B. 売上原価 (Cost of Sales)
-- ---------------------------------------------

WITH outsource_acc AS (
    INSERT INTO accounts (name, category) VALUES ('外注費(原価)', 'cost_of_sales') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY['開発委託費', 'SESパートナー費', 'デザイン委託費']) FROM outsource_acc;

WITH infra_acc AS (
    INSERT INTO accounts (name, category) VALUES ('システム原価', 'cost_of_sales') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY['AWS利用料(本番)', 'GCP利用料(本番)', 'ドメイン・SSL費用']) FROM infra_acc;

WITH purchase_acc AS (
    INSERT INTO accounts (name, category) VALUES ('商品仕入高', 'cost_of_sales') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY['ハードウェア仕入', 'ライセンス仕入']) FROM purchase_acc;


-- ---------------------------------------------
-- C. 販管費 (SGA)
-- ---------------------------------------------

-- C-1. 人件費 (社内コストの最大要素)
WITH labor_acc AS (
    INSERT INTO accounts (name, category) VALUES ('人件費', 'sga') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY['役員報酬', '給料手当', '賞与', '法定福利費', '通勤交通費']) FROM labor_acc;

-- C-2. 広告宣伝費 (攻めのコスト)
WITH ads_acc AS (
    INSERT INTO accounts (name, category) VALUES ('広告宣伝費', 'sga') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY['Web広告費', '展示会出展費', '採用媒体費', 'ノベルティ制作費']) FROM ads_acc;

-- C-3. その他経費 (守りのコスト・一般管理)
WITH admin_acc AS (
    INSERT INTO accounts (name, category) VALUES ('一般管理費', 'sga') RETURNING id
)
INSERT INTO item_types (account_id, name)
SELECT id, unnest(ARRAY[
    '地代家賃', 
    '水道光熱費', 
    '通信費', 
    '消耗品費', 
    '会議費', 
    '交際費', 
    'クラウドツール利用料(管理)', 
    '税理士・弁護士報酬'
]) FROM admin_acc;