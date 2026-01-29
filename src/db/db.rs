use rusqlite::Connection;

pub fn create_bdd() -> rusqlite::Result<()> {
    let conn = Connection::open("gestabox.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS mission (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            done INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(())
}

pub fn get_connection() -> rusqlite::Result<Connection> {
    let conn = Connection::open("gestabox.db")?;
    Ok(conn)
}