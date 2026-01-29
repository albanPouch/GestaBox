use rusqlite::{params, Connection, Result};

// La Structure (Données)
#[derive(Debug)]
pub struct Competence {
    pub id_competence: i32,
    pub libelle_competence: String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Competence(
            id_competence INT,
            libelle_competence VARCHAR(50),
            PRIMARY KEY(id_competence)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_competence: i32,
    libelle_competence: &str
) -> Result<()> {
    conn.execute(
        "INSERT INTO Competence (
            id_competence, libelle_competence
        ) VALUES (?1, ?2)",
        params![id_competence, libelle_competence],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_competence: i32) -> Result<Competence> {
    conn.query_row(
        "SELECT id_competence, libelle_competence
         FROM Competence WHERE id_competence = ?1",
        params![id_competence],
        |row| {
            Ok(Competence {
                id_competence: row.get(0)?,
                libelle_competence: row.get(1)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}