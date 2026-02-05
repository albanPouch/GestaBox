use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Occuper {
    pub id_occupation: i32,
    pub id_planning: i32,
    pub date_debut : String,
    pub date_fin: String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Occuper(
            id_occupation INT,
            id_planning INT,
            date_debut TEXT NOT NULL,
            date_fin TEXT NOT NULL,
            FOREIGN KEY (id_occupation) REFERENCES Occupation(id_occupation),
            FOREIGN KEY (id_planning) REFERENCES Planning (id_planning),
            PRIMARY KEY(id_occupation,id_planning)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_occupation: i32,
    id_planning: i32,
    date_debut : String,
    date_fin: String
) -> Result<()> {
    conn.execute(
        "INSERT INTO Occupation (
            id_occupation, id_planning, date_debut, date_fin
        ) VALUES (?1, ?2, ?3, ?4)",
        params![id_occupation, id_planning, date_debut, date_fin],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_occupation: i32) -> Result<Occuper> {
    conn.query_row(
        "SELECT id_occupation, libelle_occupation, date_debut, date_fin
         FROM Occuper WHERE id_occupation = ?1",
        params![id_occupation],
        |row| {
            Ok(Occuper {
                id_occupation: row.get(0)?,
                id_planning: row.get(1)?,
                date_debut:row.get(2)?,
                date_fin:row.get(3)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}