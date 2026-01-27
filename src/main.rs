use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("missions.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mission (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            done INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "INSERT INTO mission (title, done) VALUES (?1, ?2)",
        ("Première mission", 0),
    )?;

    Ok(())
}
