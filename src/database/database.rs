use rusqlite::{Connection, Result};

pub fn create_table() -> Result<()> {
    let conn = Connection::open("ideas.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ideas (
                id INTEGER PRIMARY KEY,
                description TEXT NOT NULL,
            )",
            [],
    )?;

    Ok(())
}

pub fn save_idea(description: &str) -> Result<()> {
    let conn = Connection::open("ideas.db")?;
    conn.execute(
        "INSERT INTO ideas (description) VALUES (?1, ?2)",
        [description],
    )?;

    Ok(())
}

pub struct Idea {
    pub(crate) id: u64,
    pub(crate) description: String,
}

pub fn get_idea(id: &str) -> Result<Idea> {
    let conn = Connection::open("ideas.db")?;
    let mut stmt = conn.prepare("SELECT id, description FROM ideas WHERE id = ?")?;
    let mut rows = stmt.query([id])?;

    if let Some(row) = rows.next()? {
        Ok(Idea {
            id: row.get(0)?,
            description: row.get(1)?,
        })
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn get_all_ideas() -> Result<Vec<Idea>> {
    let conn = Connection::open("ideas.db")?;
    let mut stmt = conn.prepare("SELECT id, description FROM ideas")?;
    let mut rows = stmt.query([])?;

    let mut ideas = Vec::new();
    while let Some(row) = rows.next()? {
        ideas.push(Idea {
            id: row.get(0)?,
            description: row.get(1)?,
        });
    }

    Ok(ideas)
}