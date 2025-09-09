CREATE EXTENSION IF NOT EXISTS pgcrypto;
-- ─────────────────────────────────────────────
-- TB_USER
-- ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS TB_USER (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    p_num VARCHAR(15) NOT NULL UNIQUE,
    -- 휴대전화
    n_name VARCHAR(50) NOT NULL UNIQUE,
    -- 닉네임
    gender SMALLINT NOT NULL CHECK (gender IN (0, 1)),
    -- 0=female, 1=male
    birth_date DATE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    status SMALLINT NOT NULL DEFAULT 0 CHECK (status IN (0, 1, 2)),
    -- 0=pending, 1=active, 2=black
    job VARCHAR(100) NOT NULL,
    city VARCHAR(50) NOT NULL,
    district VARCHAR(50) NOT NULL,
    height_cm INT NOT NULL CHECK (
        height_cm BETWEEN 50 AND 250
    ),
    body_type VARCHAR(20) NOT NULL,
    smoking VARCHAR(50) NOT NULL,
    drinking VARCHAR(50) NOT NULL,
    religion VARCHAR(50) NOT NULL,
    mbti VARCHAR(10),
    preferred_age_group VARCHAR(20),
    personalities JSONB,
    hobbies JSONB,
    introduction TEXT,
    appeal_topics JSONB
);
-- ─────────────────────────────────────────────
-- TB_USER_PROVIDER
-- ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS TB_USER_PROVIDER (
    provider_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    provider VARCHAR(32) NOT NULL,
    -- 예: kakao, naver, google, apple
    provider_user_id VARCHAR(128) NOT NULL,
    email VARCHAR(255),
    display_name VARCHAR(255),
    connected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,
    CONSTRAINT uq_user_provider UNIQUE (user_id, provider),
    CONSTRAINT uq_provider_user UNIQUE (provider, provider_user_id)
);
CREATE INDEX IF NOT EXISTS idx_user_provider_user_id ON TB_USER_PROVIDER (user_id);
-- ─────────────────────────────────────────────
-- TB_USER_PHOTO
-- ─────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS TB_USER_PHOTO (
    photo_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    photo_url VARCHAR(300) NOT NULL,
    is_main BOOLEAN NOT NULL DEFAULT FALSE,
    is_approved BOOLEAN NOT NULL DEFAULT FALSE,
    uploaded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_user_photo_user_id ON TB_USER_PHOTO (user_id);
CREATE INDEX IF NOT EXISTS idx_user_photo_uploaded_at ON TB_USER_PHOTO (uploaded_at DESC);