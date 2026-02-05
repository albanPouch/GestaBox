use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Posseder {
    pub id_intervenant: i32,
    pub id_competence: i32,
    pub preference :  i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Posseder(
            id_intervenant INT,
            id_competence INT,
            preference TEXT NOT NULL,
            FOREIGN KEY (id_intervenant) REFERENCES Intervenant(id_intervant),
            FOREIGN KEY (id_competence) REFERENCES Competence (id_competence),
            PRIMARY KEY(id_intervenant,id_competence)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_intervenant: i32,
    id_competence: i32,
    preference :  i32
) -> Result<()> {
    conn.execute(
        "INSERT INTO Posseder (
            id_intervenant, id_competence, preference
        ) VALUES (?1, ?2, ?3)",
        params![id_intervenant, id_competence, preference],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_intervenant: i32) -> Result<Posseder> {
    conn.query_row(
        "SELECT id_intervenant, id_competence, preference
         FROM Posseder WHERE id_intervenant = ?1",
        params![id_intervenant],
        |row| {
            Ok(Posseder {
                id_intervenant: row.get(0)?,
                id_competence: row.get(1)?,
                preference:row.get(2)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}