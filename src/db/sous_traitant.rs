use rusqlite::{params, Connection, Result};

#[derive(Debug, Clone)]
pub struct SousTraitant {
    pub id_sous_traitant: i32,
    pub nom_entreprise: String,
    pub metier: String,
    pub tarif_horaire: f64,
}

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

fn ensure_columns(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(SousTraitant)")?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(|c| c.ok())
        .collect();

    if !columns.contains(&"metier".to_string()) {
        conn.execute(
            "ALTER TABLE SousTraitant ADD COLUMN metier VARCHAR(50) NOT NULL DEFAULT ''",
            [],
        )?;
    }
    if !columns.contains(&"tarif_horaire".to_string()) {
        conn.execute(
            "ALTER TABLE SousTraitant ADD COLUMN tarif_horaire FLOAT NOT NULL DEFAULT 0.0",
            [],
        )?;
    }
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id: i32,
    nom_entreprise: &str,
    metier: &str,
    tarif_horaire: f64,
) -> Result<()> {
    conn.execute(
        "INSERT INTO SousTraitant (id_sous_traitant, nom_entreprise, metier, tarif_horaire)
         VALUES (?1, ?2, ?3, ?4)",
        params![id, nom_entreprise, metier, tarif_horaire],
    )?;
    Ok(())
}

pub fn update(
    conn: &Connection,
    id: i32,
    nom_entreprise: &str,
    metier: &str,
    tarif_horaire: f64,
) -> Result<()> {
    conn.execute(
        "UPDATE SousTraitant SET nom_entreprise=?2, metier=?3, tarif_horaire=?4
         WHERE id_sous_traitant=?1",
        params![id, nom_entreprise, metier, tarif_horaire],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM SousTraitant WHERE id_sous_traitant=?1",
        params![id],
    )?;
    Ok(())
}

pub fn get_by_id(conn: &Connection, id: i32) -> Result<SousTraitant> {
    conn.query_row(
        "SELECT id_sous_traitant, nom_entreprise,
                COALESCE(metier, '') as metier,
                COALESCE(tarif_horaire, 0.0) as tarif_horaire
         FROM SousTraitant WHERE id_sous_traitant=?1",
        params![id],
        |row| {
            Ok(SousTraitant {
                id_sous_traitant: row.get(0)?,
                nom_entreprise: row.get(1)?,
                metier: row.get(2)?,
                tarif_horaire: row.get(3)?,
            })
        },
    )
}

pub fn get_all(conn: &Connection) -> Result<Vec<SousTraitant>> {
    let mut stmt = conn.prepare(
        "SELECT id_sous_traitant, nom_entreprise,
                COALESCE(metier, '') as metier,
                COALESCE(tarif_horaire, 0.0) as tarif_horaire
         FROM SousTraitant ORDER BY id_sous_traitant",
    )?;
    let iter = stmt.query_map([], |row| {
        Ok(SousTraitant {
            id_sous_traitant: row.get(0)?,
            nom_entreprise: row.get(1)?,
            metier: row.get(2)?,
            tarif_horaire: row.get(3)?,
        })
    })?;
    iter.collect()
}

pub fn next_id(conn: &Connection) -> i32 {
    conn.query_row(
        "SELECT COALESCE(MAX(id_sous_traitant), 0) + 1 FROM SousTraitant",
        [],
        |row| row.get(0),
    )
    .unwrap_or(1)
}

pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(conn)?;
    ensure_columns(conn)?;
    Ok(())
}
