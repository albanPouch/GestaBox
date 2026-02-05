use rusqlite::{params, AndThenRows, Connection, Result};
use crate::db::get_connection;


pub fn init_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("
            CREATE TABLE IF NOT EXISTS Mission_historique (
            id_mission INT,
            temps_theorique VARCHAR(50),
            description VARCHAR(50),
            date_creation VARCHAR(50),
            date_debut VARCHAR(50),
            date_fin VARCHAR(50),
            derniere_modif VARCHAR(50),
            ville_mission VARCHAR(50),
            departement_mission VARCHAR(50),
            id_status INT,
            id_intervenant INT,
            date_suppression VARCHAR(50)
            );",
                 [],
    )?;
    Ok(())
}


