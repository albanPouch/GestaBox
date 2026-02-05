use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Mission {
    pub id_mission: i32,
    pub temps_theorique: String,
    pub description: String,
    pub date_creation: String ,
    pub date_debut: String,
    pub date_fin: String ,
    pub derniere_modif: String,
    pub ville_mission: String ,
    pub departement_mission : String,
    pub id_status: i32,
    pub id_intervenant: i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Mission(
        id_mission INT,
        temps_theorique VARCHAR(50),
        description VARCHAR(50),
        date_creation VARCHAR(50),
        date_debut VARCHAR(50),
        date_fin VARCHAR(50),
        derniere_modif VARCHAR(50),
        ville_mission VARCHAR(50),
        departement_mission VARCHAR(50),
        id_status INT NOT NULL,
        id_intervenant INT NOT NULL,
        FOREIGN KEY(id_status) REFERENCES status(id_status),
        FOREIGN KEY(id_intervenant) REFERENCES Intervenant(id_intervant),
        PRIMARY KEY(id_mission)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_mission: i32,
    temps_theorique:  &str,
    description:  &str,
    date_creation:  &str,
    date_debut : &str,
    date_fin :  &str,
    derniere_modif:  &str,
    ville_mission :  &str,
    departement_mission :  &str,
    id_status : i32,
    id_intervenant: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Mission (
            id_mission, temps_theorique, description, date_creation, date_debut, date_fin, derniere_modif, ville_mission , departement_mission, id_status, id_intervenant
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![id_mission, temps_theorique, description,
            date_creation, date_debut, date_fin ,derniere_modif,
            ville_mission, departement_mission, id_status , id_intervenant
        ],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_mission: i32) -> Result<Mission> {
    conn.query_row(
        "SELECT id_mission, temps_theorique, description, date_creation, date_debut, date_fin, derniere_modif,
        ville_mission , departement_mission, id_status, id_intervenant
         FROM Mission WHERE id_mission = ?1",
        params![id_mission],
        |row| {
            Ok(Mission {
                id_mission: row.get(0)?,
                temps_theorique: row.get(1)?,
                description: row.get(2)?,
                date_creation: row.get(3)?,
                date_debut: row.get(4)?,
                date_fin: row.get(5)?,
                derniere_modif:row.get(6)?,
                ville_mission: row.get(7)?,
                departement_mission: row.get(8)?,
                id_status: row.get(9)?,
                id_intervenant: row.get(10)?,
            })
        },
    )
}
// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}