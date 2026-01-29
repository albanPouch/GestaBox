use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Planning {
    pub id_planning: i32,
    pub annee: String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Planning(
            id_planning INT,
            annee VARCHAR(50),
            PRIMARY KEY(id_planning)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_planning: i32,
    annee: &str
) -> Result<()> {
    conn.execute(
        "INSERT INTO Planning (
            id_planning, annee
        ) VALUES (?1, ?2)",
        params![id_planning, annee],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_planning: i32) -> Result<Planning> {
    conn.query_row(
        "SELECT id_planning, annee
         FROM Planning WHERE id_status = ?1",
        params![id_planning],
        |row| {
            Ok(Planning {
                id_planning: row.get(0)?,
                annee: row.get(1)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}