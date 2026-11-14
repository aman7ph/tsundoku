-- Folders can no longer be nested: a category lives directly inside a collection.
-- Remove any folder that still has a parent (its links go with it), then drop the column.
DELETE FROM categories WHERE parent_id IS NOT NULL;

ALTER TABLE categories DROP COLUMN parent_id;