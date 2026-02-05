use rusqlite::Connection;

pub fn create_bdd() -> rusqlite::Result<()> {
    let conn = Connection::open("gestabox.db")?;

    let result = conn.execute(
        "CREATE TABLE IF NOT EXISTS mission (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            done INTEGER NOT NULL
        )",
        [],
    );

    match result {
        Ok(_) => {
            println!("Création des tables OK !");
            Ok(())
        }
        Err(e) => {
            println!(" Erreur BD : {}", e);
            Err(e)
        }
    }
}