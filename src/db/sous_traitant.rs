use std::ffi::c_float;
use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct SousTraitant {
    pub id_sous_traitant: i32,
    pub nom_entreprise: String,
    pub taux_journalier: c_float,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS SousTraitant(
            id_sous_traitant INT,
            nom_entreprise VARCHAR(50),
            taux_journalier float,
            PRIMARY KEY(id_sous_traitant)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_sous_traitant: i32,
    nom_entreprise: &str,
    taux_horaire: &c_float,
) -> Result<()> {
    conn.execute(
        "INSERT INTO SousTraitant (
            id_sous_traitant, nom_entreprise, taux_journalier
        ) VALUES (?1, ?2, ?3)",
        params![id_sous_traitant, nom_entreprise, taux_horaire],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_sous_traitant: i32) -> Result<SousTraitant> {
    conn.query_row(
        "SELECT id_sous_traitant, nom_entreprise, taux_journalier
         FROM SousTraitant WHERE id_sous_traitant = ?1",
        params![id_sous_traitant],
        |row| {
            Ok(SousTraitant {
                id_sous_traitant: row.get(0)?,
                nom_entreprise: row.get(1)?,
                taux_journalier: row.get(2)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}