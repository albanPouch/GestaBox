use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Demander {
    pub id_competence: i32,
    pub id_activite :  i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Demander(
            id_activite INT,
            id_competence INT,
            FOREING KEY id_activite REFERENCES Activite (id_activite),
            FOREING KEY id_competence REFERENCES Competence (id_competence),
            PRIMARY KEY(id_activite,id_competence)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_activite: i32,
    id_competence: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Demander (
            id_occupation, id_competence
        ) VALUES (?1, ?2)",
        params![id_activite, id_competence],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_activite: i32) -> Result<Demander> {
    conn.query_row(
        "SELECT id_activite, id_competence
         FROM Demander WHERE id_activite = ?1",
        params![id_activite],
        |row| {
            Ok(Demander {
                id_activite: row.get(0)?,
                id_competence: row.get(1)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}