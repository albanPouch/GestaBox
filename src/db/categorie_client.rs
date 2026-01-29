use std::os::raw::c_float;
use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct CategorieClient {
    pub id_categorie: i32,
    pub libelle_categorie: String,
    pub taux_horaire : c_float,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS CategorieClient(
            id_categorie INT,
            libelle_categorie VARCHAR(50),
            taux_horaire c_float,
            PRIMARY KEY(id_categorie)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_categorie: i32,
    libelle_categorie: &str,
    taux_horaire : c_float,

) -> Result<()> {
    conn.execute(
        "INSERT INTO CategorieClient (
            id_categorie, libelle_categorie, taux_horaire
        ) VALUES (?1, ?2)",
        params![id_categorie, libelle_categorie,taux_horaire],
    )?;
    Ok(())
}


// 4. Lecture
pub fn get_by_id(conn: &Connection, id_categorie: i32) -> Result<CategorieClient> {
    conn.query_row(
        "SELECT id_categorie, libelle_categorie,taux_horaire
         FROM CategorieClient WHERE id_categorie = ?1",
        params![id_categorie],
        |row| {
            Ok(CategorieClient {
                id_categorie: row.get(0)?,
                libelle_categorie: row.get(1)?,
                taux_horaire: row.get(2)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}