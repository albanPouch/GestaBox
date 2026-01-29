use rusqlite::Connection;
pub use db:: create_bdd;

// On déclare les modules
pub mod db;
mod client;
mod status;
mod occupation;
mod planning;
mod categorie_client;
mod employee;
mod type_mission;
mod competence;
mod intervenant;
mod sous_traitant;
mod activite;
mod mission;

pub fn get_connection() -> rusqlite::Result<Connection> {
    let conn = Connection::open("gestabox.db")?;
    Ok(conn)
}

pub fn init_gestabox_db() -> rusqlite::Result<()> {
    create_bdd()?;

    // On récupére la connexion
    let conn = get_connection()?;

    // On initialise les tables
    client::init_db(&conn)?;
    status::init_db(&conn)?;
    Ok(())
}