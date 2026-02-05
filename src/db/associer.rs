use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Associer {
    pub id_client: i32,
    pub id_mission: i32,
    pub date_demande :  String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Associer(
            id_client INT,
            id_mission INT,
            date_demande TEXT NOT NULL,
            FOREIGN KEY (id_client) REFERENCES Client(id_client),
            FOREIGN KEY (id_mission) REFERENCES Mission (id_mission),
            PRIMARY KEY(id_client,id_mission)
        )",
        [],
    )?;
    Ok(())
}

pub fn create(
    conn: &Connection,
    id_client: i32,
    id_mission: i32,
    date_demande :  String
) -> Result<()> {
    conn.execute(
        "INSERT INTO Associer (
            id_client, id_mission, date_demande
        ) VALUES (?1, ?2, ?3)",
        params![id_client, id_mission, date_demande],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_client: i32) -> Result<Associer> {
    conn.query_row(
        "SELECT id_client, id_mission, date_demande
         FROM Associer WHERE id_client = ?1",
        params![id_client],
        |row| {
            Ok(Associer {
                id_client: row.get(0)?,
                id_mission: row.get(1)?,
                date_demande:row.get(2)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}