CREATE TABLE mission_messages (
    id SERIAL PRIMARY KEY,
    mission_id INTEGER NOT NULL REFERENCES missions(id),
    user_id INTEGER REFERENCES brawlers(id),
    content TEXT NOT NULL,
    "type" VARCHAR(50) NOT NULL, -- 'chat' or 'system'
    created_at TIMESTAMP NOT NULL DEFAULT now()
);
