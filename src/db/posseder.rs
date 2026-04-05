use rusqlite::{params, Connection, Result};

// La Structure (Données)
#[derive(Debug)]
pub struct Posseder {
    pub id_intervenant: i32,
    pub id_competence: i32,
    pub niveau: i32,
    pub preference :  i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Posseder(
            id_intervenant INT,
            id_competence INT,
            niveau INT NOT NULL DEFAULT 0,
            preference INT NOT NULL,
            FOREIGN KEY (id_intervenant) REFERENCES Intervenant(id_intervant),
            FOREIGN KEY (id_competence) REFERENCES Competence (id_competence),
            PRIMARY KEY(id_intervenant,id_competence)
        )",
        [],
    )?;
    Ok(())
}

fn ensure_niveau_column(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("PRAGMA table_info(Posseder)")?;
    let columns = stmt.query_map([], |row| row.get::<_, String>(1))?;

    let mut has_niveau = false;
    for column in columns {
        if column? == "niveau" {
            has_niveau = true;
            break;
        }
    }

    if !has_niveau {
        conn.execute(
            "ALTER TABLE Posseder ADD COLUMN niveau INT NOT NULL DEFAULT 0",
            [],
        )?;
    }

    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_intervenant: i32,
    id_competence: i32,
    niveau: i32,
    preference :  i32
) -> Result<()> {
    conn.execute(
        "INSERT INTO Posseder (
            id_intervenant, id_competence, niveau, preference
        ) VALUES (?1, ?2, ?3, ?4)",
        params![id_intervenant, id_competence, niveau, preference],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_intervenant: i32) -> Result<Posseder> {
    conn.query_row(
        "SELECT id_intervenant, id_competence, niveau, preference
         FROM Posseder WHERE id_intervenant = ?1",
        params![id_intervenant],
        |row| {
            Ok(Posseder {
                id_intervenant: row.get(0)?,
                id_competence: row.get(1)?,
                niveau: row.get(2)?,
                preference:row.get(3)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(conn)?;
    ensure_niveau_column(conn)?;
    Ok(())
}
