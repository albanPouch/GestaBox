use rusqlite::{params, Connection, Result};

// La Structure (Données)
#[derive(Debug)]
pub struct Status {
    pub id_status: i32,
    pub libelle_status: String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Status(
            id_status INT,
            libelle_status VARCHAR(50),
            PRIMARY KEY(id_status)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_status: i32,
    libelle_status: &str
) -> Result<()> {
    conn.execute(
        "INSERT INTO Status (
            id_status, libelle_status
        ) VALUES (?1, ?2)",
        params![id_status, libelle_status],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_status: i32) -> Result<Status> {
    conn.query_row(
        "SELECT id_status, libelle_status
         FROM Status WHERE id_status = ?1",
        params![id_status],
        |row| {
            Ok(Status {
                id_status: row.get(0)?,
                libelle_status: row.get(1)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}