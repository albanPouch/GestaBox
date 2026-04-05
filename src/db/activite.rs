use rusqlite::{params, Connection, Result};

// La Structure (Données)
#[derive(Debug)]
pub struct Activite {
    pub id_activite: i32,
    pub libelle_activite: String,
    pub temps_tache_theorique: String,
    pub prioritaire: bool ,
    pub faisable_en_teletravail : bool,
    pub id_status: i32,
    pub id_mission: i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Activite(
            id_activite INT,
            libelle_activite VARCHAR(50),
            temps_tache_theorique VARCHAR(50),
            prioritaire Boolean,
            faisable_en_teletravail Boolean,
            id_status INT,
            id_mission INT,
            FOREIGN KEY(id_status) REFERENCES Status(id_status),
            FOREIGN KEY(id_mission) REFERENCES  Mission (id_mission),
            PRIMARY KEY(id_activite)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_activite: i32,
    libelle_activite: &str,
    temps_tache_theorique: &str,
    prioritaire: bool,
    faisable_en_teletravail: bool,
    id_status : i32,
    id_mission: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Activite (
            id_activite, libelle_activite, temps_tache_theorique, prioritaire, faisable_en_teletravail, id_status, id_mission
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id_activite, libelle_activite, temps_tache_theorique,
            prioritaire, faisable_en_teletravail, id_status, id_mission],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_activite: i32) -> Result<Activite> {
    conn.query_row(
        "SELECT id_activite, libelle_activite, temps_tache_theorique,
         prioritaire, faisable_en_teletravail , id_status, id_mission
         FROM Activite WHERE id_activite = ?1",
        params![id_activite],
        |row| {
            Ok(Activite {
                id_activite: row.get(0)?,
                libelle_activite: row.get(1)?,
                temps_tache_theorique: row.get(2)?,
                prioritaire: row.get(3)?,
                faisable_en_teletravail: row.get(4)?,
                id_status: row.get(5)?,
                id_mission:row.get(6)?,
            })
        },
    )
}
// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}