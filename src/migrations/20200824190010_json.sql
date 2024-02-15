CREATE TABLE IF NOT EXISTS payloads
(
    id BIGSERIAL PRIMARY KEY,
    number BIGINT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    labels TEXT[],
    creator TEXT NOT NULL,
    essence TEXT
);