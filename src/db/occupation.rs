use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Occupation {
    pub id_occupation: i32,
    pub libelle_occupation: String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Occupation(
            id_occupation INT,
            libelle_occupation VARCHAR(50),
            PRIMARY KEY(id_occupation)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_occupation: i32,
    libelle_occupation: &str
) -> Result<()> {
    conn.execute(
        "INSERT INTO Occupation (
            id_occupation, libelle_occupation
        ) VALUES (?1, ?2)",
        params![id_occupation, libelle_occupation],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_occupation: i32) -> Result<Occupation> {
    conn.query_row(
        "SELECT id_occupation, libelle_occupation
         FROM Occupation WHERE id_occupation = ?1",
        params![id_occupation], 
        |row| {
            Ok(Occupation {
                id_occupation: row.get(0)?,
                libelle_occupation: row.get(1)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}