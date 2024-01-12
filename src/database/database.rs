use rusqlite::{Connection, Result};

pub fn create_table() -> Result<()> {
    let conn = Connection::open("ideas.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ideas (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
                content TEXT
            )",
            [],
    )?;

    Ok(())
}

pub fn save_idea(description: &str, content: Option<String>) -> Result<()> {
    let conn = Connection::open("ideas.db")?;
    conn.execute(
        "INSERT INTO ideas (description, content) VALUES (?1, ?2)",
        [description, content],
    )?;

    Ok(())
}