-- Add migration script here
CREATE TABLE sections (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    position    INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE categories (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    section_id  UUID NOT NULL REFERENCES sections(id) ON DELETE CASCADE,
    parent_id   UUID REFERENCES categories(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    position    INT NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE resources (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    category_id  UUID NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    url          TEXT NOT NULL,
    title        TEXT,
    description  TEXT,
    visited      BOOLEAN NOT NULL DEFAULT FALSE,
    shareable    BOOLEAN NOT NULL DEFAULT FALSE,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sections_user ON sections(user_id);
CREATE INDEX idx_categories_user ON categories(user_id);
CREATE INDEX idx_categories_section ON categories(section_id);
CREATE INDEX idx_categories_parent ON categories(parent_id);
CREATE INDEX idx_resources_user ON resources(user_id);
CREATE INDEX idx_resources_category ON resources(category_id);