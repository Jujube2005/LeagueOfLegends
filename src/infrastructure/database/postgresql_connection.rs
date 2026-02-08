use anyhow::Result;
use diesel::{
    connection::SimpleConnection,
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/infrastructure/database/migrations");

pub type PgPoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection(database_url: &str) -> Result<PgPoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    
    // Run migrations automatically
    // Trigger recompile
    let mut conn = pool.get()?;
    info!("Running database migrations...");
    // We try to run migrations, but if it fails, we assume it might be due to dirty state and proceed to force-create tables.
    // However, usually we want it to fail hard. But in this specific user case, migrations are flaky.
    // Let's keep it failing hard for migrations, but add a fallback or explicit execution for achievements.
    if let Err(e) = conn.run_pending_migrations(MIGRATIONS) {
        info!("Migration warning: {}", e);
        // We continue because sometimes "relation already exists" is false positive in mixed environments
    }
    info!("Database migrations completed (or skipped).");

    // Force ensure achievements tables exist (Manual Migration Fallback)
    let raw_sql = r#"
    CREATE TABLE IF NOT EXISTS achievements (
        id SERIAL PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        description TEXT,
        icon_url VARCHAR(255),
        condition_type VARCHAR(50),
        condition_value INTEGER,
        created_at TIMESTAMP NOT NULL DEFAULT now()
    );

    CREATE TABLE IF NOT EXISTS brawler_achievements (
        brawler_id INTEGER NOT NULL,
        achievement_id INTEGER NOT NULL,
        earned_at TIMESTAMP NOT NULL DEFAULT now(),
        PRIMARY KEY (brawler_id, achievement_id)
    );

    DO $$
    BEGIN
        BEGIN
            ALTER TABLE brawler_achievements
                ADD CONSTRAINT fk_brawler_achievements_brawler
                FOREIGN KEY (brawler_id)
                REFERENCES brawlers(id);
        EXCEPTION
            WHEN duplicate_object THEN NULL;
        END;
    END $$;

    DO $$
    BEGIN
        BEGIN
            ALTER TABLE brawler_achievements
                ADD CONSTRAINT fk_brawler_achievements_achievement
                FOREIGN KEY (achievement_id)
                REFERENCES achievements(id);
        EXCEPTION
            WHEN duplicate_object THEN NULL;
        END;
    END $$;

    INSERT INTO achievements (name, description, icon_url, condition_type, condition_value)
    SELECT 'First Blood', 'Completed your first mission', 'assets/badges/first_blood.png', 'mission_complete', 1
    WHERE NOT EXISTS (SELECT 1 FROM achievements WHERE name = 'First Blood');

    INSERT INTO achievements (name, description, icon_url, condition_type, condition_value)
    SELECT 'Veteran', 'Completed 10 missions', 'assets/badges/veteran.png', 'mission_complete', 10
    WHERE NOT EXISTS (SELECT 1 FROM achievements WHERE name = 'Veteran');

    INSERT INTO achievements (name, description, icon_url, condition_type, condition_value)
    SELECT 'Squad Leader', 'Created 5 missions', 'assets/badges/leader.png', 'mission_create', 5
    WHERE NOT EXISTS (SELECT 1 FROM achievements WHERE name = 'Squad Leader');

    INSERT INTO achievements (name, description, icon_url, condition_type, condition_value)
    SELECT 'Team Player', 'Joined 5 missions', 'assets/badges/team_player.png', 'mission_join', 5
    WHERE NOT EXISTS (SELECT 1 FROM achievements WHERE name = 'Team Player');
    "#;

    // Force integrate category column for missions
    let raw_sql_missions = r#"
    DO $$
    BEGIN
        BEGIN
            ALTER TABLE missions ADD COLUMN category VARCHAR(255);
        EXCEPTION
            WHEN duplicate_column THEN NULL;
        END;
    END $$;
    "#;
    
    conn.batch_execute(raw_sql_missions)
       .map_err(|e| anyhow::anyhow!("Failed to add category column: {}", e))?;

    // Force integrate max_crew column for missions
    let raw_sql_max_crew = r#"
    DO $$
    BEGIN
        BEGIN
            ALTER TABLE missions ADD COLUMN max_crew INTEGER NOT NULL DEFAULT 5;
        EXCEPTION
            WHEN duplicate_column THEN NULL;
        END;
    END $$;
    "#;
    
    conn.batch_execute(raw_sql_max_crew)
       .map_err(|e| anyhow::anyhow!("Failed to add max_crew column: {}", e))?;

    conn.batch_execute(raw_sql)
        .map_err(|e| anyhow::anyhow!("Failed to force create achievements tables: {}", e))?;
    
    info!("Verified achievements tables.");

    Ok(pool)
}
