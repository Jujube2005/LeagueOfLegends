-- Your SQL goes here
DO $$
BEGIN
    BEGIN
        ALTER TABLE brawlers ADD COLUMN display_name VARCHAR(50) NOT NULL DEFAULT 'Unknown';
    EXCEPTION
        WHEN duplicate_column THEN NULL;
    END;

    BEGIN
        ALTER TABLE brawlers ADD COLUMN avatar_url VARCHAR(512);
    EXCEPTION
        WHEN duplicate_column THEN NULL;
    END;

    BEGIN
        ALTER TABLE brawlers ADD COLUMN avatar_public_id VARCHAR(255);
    EXCEPTION
        WHEN duplicate_column THEN NULL;
    END;
END $$;
