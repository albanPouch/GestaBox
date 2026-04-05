use rusqlite::{params, Connection, Result};

// La Structure (Données)
#[derive(Debug)]
pub struct Intervenir {
    pub id_intervenant: i32,
    pub id_activite: i32,
    pub date_intervention :  String,
    pub heure_intervention : String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Intervenir(
            id_intervenant INT,
            id_activite INT,
            date_intervention TEXT NOT NULL,
            heure_intervention TEXT NOT NULL,
            FOREIGN KEY (id_intervenant) REFERENCES Intervenant(id_intervant),
            FOREIGN KEY (id_activite) REFERENCES Activite (id_activite),
            PRIMARY KEY(id_intervenant,id_activite)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_intervenant: i32,
    id_activite: i32,
    date_intervention :  String,
    heure_intervention :  String,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Intervenir (
            id_intervenant, id_activite, date_intervention, heure_intervention
        ) VALUES (?1, ?2, ?3, ?4)",
        params![id_intervenant, id_activite, date_intervention, heure_intervention],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_intervenant: i32) -> Result<Intervenir> {
    conn.query_row(
        "SELECT id_intervenant, id_activite, date_intervention , heure_intervention
         FROM Intervenir WHERE id_intervenant = ?1",
        params![id_intervenant],
        |row| {
            Ok(Intervenir {
                id_intervenant: row.get(0)?,
                id_activite: row.get(1)?,
                date_intervention:row.get(2)?,
                heure_intervention:row.get(3)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}