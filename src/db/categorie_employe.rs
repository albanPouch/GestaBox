use std::os::raw::c_float;
use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct CategorieEmploye {
    pub id_categorie: i32,
    pub libelle_categorie: String,
    pub taux_journalier : c_float,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS CategorieEmploye(
            id_categorie INT,
            libelle_categorie VARCHAR(50),
            taux_journalier c_float,
            PRIMARY KEY(id_categorie)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_categorie: i32,
    libelle_categorie: &str,
    taux_horaire : c_float,

) -> Result<()> {
    conn.execute(
        "INSERT INTO CategorieEmploye (
            id_categorie, libelle_categorie, taux_journalier
        ) VALUES (?1, ?2, ?3)",
        params![id_categorie, libelle_categorie,taux_horaire],
    )?;
    Ok(())
}


// 4. Lecture
pub fn get_by_id(conn: &Connection, id_categorie: i32) -> Result<CategorieEmploye> {
    conn.query_row(
        "SELECT id_categorie, libelle_categorie,taux_horaire
         FROM CategorieEmploye WHERE id_categorie = ?1",
        params![id_categorie],
        |row| {
            Ok(CategorieEmploye {
                id_categorie: row.get(0)?,
                libelle_categorie: row.get(1)?,
                taux_journalier: row.get(2)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}