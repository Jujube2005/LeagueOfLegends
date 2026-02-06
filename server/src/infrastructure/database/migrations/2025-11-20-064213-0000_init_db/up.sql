-- Your SQL goes here
CREATE TABLE IF NOT EXISTS brawlers (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    "password" VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE IF NOT EXISTS missions (
    id SERIAL PRIMARY KEY,
    "name" VARCHAR(255) NOT NULL,
    "description" TEXT,
    "status" VARCHAR(255) NOT NULL,
    chief_id INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    deleted_at TIMESTAMP
);

CREATE TABLE IF NOT EXISTS crew_memberships (
    mission_id INTEGER NOT NULL,
    brawler_id INTEGER NOT NULL,
    joined_at TIMESTAMP NOT NULL DEFAULT now(),
    PRIMARY KEY (mission_id, brawler_id)
);

DO $$
BEGIN
    BEGIN
        ALTER TABLE missions ADD CONSTRAINT fk_chief FOREIGN KEY (chief_id) REFERENCES brawlers(id);
    EXCEPTION
        WHEN duplicate_object THEN NULL;
    END;
END $$;

DO $$
BEGIN
    BEGIN
        ALTER TABLE crew_memberships ADD CONSTRAINT fk_mission FOREIGN KEY (mission_id) REFERENCES missions(id);
    EXCEPTION
        WHEN duplicate_object THEN NULL;
    END;
END $$;

DO $$
BEGIN
    BEGIN
        ALTER TABLE crew_memberships ADD CONSTRAINT fk_brawler FOREIGN KEY (brawler_id) REFERENCES brawlers(id);
    EXCEPTION
        WHEN duplicate_object THEN NULL;
    END;
END $$;

DO $$
BEGIN
    BEGIN
        PERFORM diesel_manage_updated_at('missions');
    EXCEPTION
        WHEN duplicate_object THEN NULL;
    END;
END $$;

DO $$
BEGIN
    BEGIN
        PERFORM diesel_manage_updated_at('brawlers');
    EXCEPTION
        WHEN duplicate_object THEN NULL;
    END;
END $$;
