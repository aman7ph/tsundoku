-- Add migration script here
CREATE TABLE platforms (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key         TEXT,
    name        TEXT NOT NULL,
    icon        TEXT,
    position    INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE accounts (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    platform_id UUID NOT NULL REFERENCES platforms(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    position    INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (platform_id, name)
);

CREATE TABLE social_categories (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    account_id  UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    icon        TEXT,
    position    INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE social_links (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id  UUID NOT NULL REFERENCES social_categories(id) ON DELETE CASCADE,
    url          TEXT NOT NULL,
    title        TEXT,
    description  TEXT,
    shareable    BOOLEAN NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- One platform per key per user (custom platforms with NULL key are allowed many times)
CREATE UNIQUE INDEX idx_platforms_user_key ON platforms(user_id, key) WHERE key IS NOT NULL;

CREATE INDEX idx_platforms_user ON platforms(user_id);
CREATE INDEX idx_accounts_user ON accounts(user_id);
CREATE INDEX idx_accounts_platform ON accounts(platform_id);
CREATE INDEX idx_social_categories_user ON social_categories(user_id);
CREATE INDEX idx_social_categories_account ON social_categories(account_id);
CREATE INDEX idx_social_links_user ON social_links(user_id);
CREATE INDEX idx_social_links_category ON social_links(category_id);