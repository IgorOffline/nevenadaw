CREATE EXTENSION IF NOT EXISTS vector;

CREATE TABLE IF NOT EXISTS vector_store (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    content text NOT NULL,
    metadata jsonb DEFAULT '{}'::jsonb,
    embedding vector(384)
);

CREATE INDEX IF NOT EXISTS idx_vector_store_hnsw_cosine 
ON vector_store 
USING hnsw (embedding vector_cosine_ops)
WITH (m = 24, ef_construction = 100);