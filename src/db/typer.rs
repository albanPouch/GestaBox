use rusqlite::{params, Connection, Result};

// La Structure (Données)
#[derive(Debug)]
pub struct Typer {
    pub id_type_mission: i32,
    pub id_mission: i32,
    pub importance: i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Typer(
            id_type_mission INT,
            id_mission INT,
            importance INT,
            FOREIGN KEY (id_type_mission) REFERENCES type_mission (id_type_mission),
            FOREIGN KEY (id_mission) REFERENCES Mission (id_mission),
            PRIMARY KEY(id_type_mission,id_mission)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_type_mission: i32,
    id_mission: i32,
    importance: i32
) -> Result<()> {
    conn.execute(
        "INSERT INTO Typer (
            id_type_mission, id_mission, importance
        ) VALUES (?1, ?2, ?3)",
        params![id_type_mission, id_mission, importance],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_type_mission: i32) -> Result<Typer> {
    conn.query_row(
        "SELECT id_type_mission, id_mission , importance
         FROM Typer WHERE id_type_mission = ?1",
        params![id_type_mission],
        |row| {
            Ok(Typer {
                id_type_mission: row.get(0)?,
                id_mission: row.get(1)?,
                importance:row.get(2)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}