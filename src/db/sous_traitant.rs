use std::ffi::c_float;
use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Sous_traitant {
    pub id_sous_traitant: i32,
    pub nom_entreprise: String,
    pub taux_horaire: c_float,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Sous_traitant(
            id_sous_traitant INT,
            nom_entreprise VARCHAR(50),
            taux_horaire float,
            PRIMARY KEY(id_sous_traitant),
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_sous_traitant: i32,
    nom_entreprise: &str,
    taux_horaire: &c_float,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Sous_traitant (
            id_sous_traitant, nom_entreprise, taux_horaire
        ) VALUES (?1, ?2, ?3)",
        params![id_sous_traitant, nom_entreprise, taux_horaire],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id: i32) -> Result<Sous_traitant> {
    conn.query_row(
        "SELECT id_sous_traitant, nom_entreprise, taux_horaire
         FROM Sous_traitant WHERE id_sous_traitant = ?1",
        params![id],
        |row| {
            Ok(Sous_traitant {
                id_sous_traitant: row.get(0)?,
                nom_entreprise: row.get(1)?,
                taux_horaire: row.get(2)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}