use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct type_mission {
    pub id_type_mission: i32,
    pub libelle_type_mission: String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS type_mission(
            id_type_mission INT,
            libelle_type_mission VARCHAR(50),
            PRIMARY KEY(id_type_mission)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_type_mission: i32,
    libelle_type_mission: &str,

) -> Result<()> {
    conn.execute(
        "INSERT INTO type_mission (
            id_type_mission, libelle_type_mission
        ) VALUES (?1, ?2)",
        params![id_type_mission, libelle_type_mission],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_type_mission: i32) -> Result<type_mission> {
    conn.query_row(
        "SELECT id_type_mission, libelle_type_mission
         FROM type_mission WHERE id_type_mission = ?1",
        params![id_type_mission],
        |row| {
            Ok(type_mission {
                id_type_mission: row.get(0)?,
                libelle_type_mission: row.get(1)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}
