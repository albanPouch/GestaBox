#![allow(dead_code)]
#![allow(non_camel_case_types)]

use rusqlite::Connection;

// On déclare les modules
pub mod db;
pub mod seeds;
pub mod client;
mod status;
mod occupation;
mod planning;
mod categorie_employe;
pub(crate) mod employee;
mod type_mission;
mod competence;
mod intervenant;
mod sous_traitant;
mod activite;
pub(crate) mod mission;
mod occuper;
mod typer;
mod posseder;
mod demander;
mod associer;
mod intervenir;
mod trigger;
mod mission_historique;

pub fn get_connection() -> rusqlite::Result<Connection> {
    let conn = Connection::open("gestabox.db")?;
    Ok(conn)
}

pub fn init_gestabox_db() -> rusqlite::Result<()> {
    // On récupére la connexion
    let conn = get_connection()?;

    // On initialise les tables
    client::init_db(&conn)?;
    status::init_db(&conn)?;
    activite::init_db(&conn)?;
    mission::init_db(&conn)?;
    employee::init_db(&conn)?;
    type_mission::init_db(&conn)?;
    competence::init_db(&conn)?;
    intervenant::init_db(&conn)?;
    sous_traitant::init_db(&conn)?;
    occupation::init_db(&conn)?;
    planning::init_db(&conn)?;
    occuper::init_db(&conn)?;
    typer::init_db(&conn)?;
    posseder::init_db(&conn)?;
    demander::init_db(&conn)?;
    associer::init_db(&conn)?;
    intervenir::init_db(&conn)?;
    categorie_employe::init_db(&conn)?;
    
    // Seed data
    seeds::seed_all(&conn)?;
    
    Ok(())
}