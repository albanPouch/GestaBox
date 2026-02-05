use rusqlite::{Connection, Result, Error, ErrorCode};
use crate::db::{activite, associer, categorie_employe, client, competence, demander, employee, intervenant, intervenir, mission, occupation, occuper, planning, posseder, sous_traitant, status, type_mission, typer};

// configuration pour activiter/déactiviter facilement de seed
pub const SEEDING_ENABLED: bool = true;

fn try_insert(result: Result<()>) -> Result<()> {
    match result {
        Ok(()) => Ok(()),
        Err(Error::SqliteFailure(e, _)) => {
            if e.code == ErrorCode::ConstraintViolation {
                // Ignore duplicate entries
                Ok(())
            } else {
                Err(Error::SqliteFailure(e, None))
            }
        }
        Err(e) => Err(e),
    }
}

pub fn seed_all(conn: &Connection) -> Result<()> {
    if !SEEDING_ENABLED {
        println!("Seeding is disabled.");
        return Ok(());
    }

    println!("Starting database seeding...");

    // 1. Independent Tables
    println!("Seeding Clients...");
    seed_clients(conn)?;
    println!("Seeding Status...");
    seed_status(conn)?;
    println!("Seeding CategorieEmploye...");
    seed_categorie_employe(conn)?;
    println!("Seeding SousTraitant...");
    seed_sous_traitant(conn)?;
    println!("Seeding Occupation...");
    seed_occupation(conn)?;
    println!("Seeding Planning...");
    seed_planning(conn)?;
    println!("Seeding Competence...");
    seed_competence(conn)?;
    println!("Seeding TypeMission...");
    seed_type_mission(conn)?;

    // 2. Dependent Tables (Level 1)
    println!("Seeding Employees...");
    seed_employees(conn)?; // Depends on CategorieEmploye
    println!("Seeding Intervenants...");
    seed_intervenants(conn)?; // Depends on Planning, Employee, SousTraitant

    // 3. Dependent Tables (Level 2)
    println!("Seeding Missions...");
    seed_missions(conn)?; // Depends on Status, Intervenant

    // 4. Dependent Tables (Level 3)
    println!("Seeding Activites...");
    seed_activites(conn)?; // Depends on Status, Mission
    println!("Seeding Typer...");
    seed_typer(conn)?; // Depends on TypeMission, Mission
    println!("Seeding Associer...");
    seed_associer(conn)?; // Depends on Client, Mission
    println!("Seeding Posseder...");
    seed_posseder(conn)?; // Depends on Intervenant, Competence

    // 5. Dependent Tables (Level 4)
    println!("Seeding Intervenir...");
    seed_intervenir(conn)?; // Depends on Intervenant, Activite
    println!("Seeding Demander...");
    seed_demander(conn)?; // Depends on Activite, Competence
    println!("Seeding Occuper...");
    seed_occuper(conn)?; // Depends on Occupation, Planning

    println!("Database seeding completed successfully.");
    Ok(())
}

fn seed_clients(conn: &Connection) -> Result<()> {
    // id, nom, prenom, raison_social, tel
    try_insert(client::insert(conn, 1, "Dupont", "Jean", Some("Dupont SA"), "0102030405"))?;
    try_insert(client::insert(conn, 2, "Durand", "Marie", None, "0607080910"))?;
    try_insert(client::insert(conn, 3, "Bernard", "John", None, "0607080911"))?;
    Ok(())
}

fn seed_status(conn: &Connection) -> Result<()> {
    // id, libelle
    try_insert(status::insert(conn, 1, "En cours"))?;
    try_insert(status::insert(conn, 2, "Terminé"))?;
    try_insert(status::insert(conn, 3, "Annulé"))?;
    Ok(())
}

fn seed_categorie_employe(conn: &Connection) -> Result<()> {
    // id, libelle, taux
    try_insert(categorie_employe::insert(conn, 1, "Junior", 150.0f32))?;
    try_insert(categorie_employe::insert(conn, 2, "Senior", 400.0f32))?;
    try_insert(categorie_employe::insert(conn, 3, "Expert", 350.0f32))?;
    Ok(())
}

fn seed_sous_traitant(conn: &Connection) -> Result<()> {
    // id, nom, taux
    try_insert(sous_traitant::insert(conn, 1, "SousTreat SARL", &200.0f32))?;
    Ok(())
}

fn seed_occupation(conn: &Connection) -> Result<()> {
    // id, libelle
    try_insert(occupation::insert(conn, 1, "Réunion"))?;
    try_insert(occupation::insert(conn, 2, "Développement"))?;
    try_insert(occupation::insert(conn, 3, "Archicteture"))?;
    Ok(())
}

fn seed_planning(conn: &Connection) -> Result<()> {
    // id, annee
    try_insert(planning::insert(conn, 1, "2023"))?;
    try_insert(planning::insert(conn, 2, "2024"))?;
    Ok(())
}

fn seed_competence(conn: &Connection) -> Result<()> {
    // id, libelle
    try_insert(competence::insert(conn, 1, "Rust"))?;
    try_insert(competence::insert(conn, 2, "SQL"))?;
    try_insert(competence::insert(conn, 3, "Management"))?;
    Ok(())
}

fn seed_type_mission(conn: &Connection) -> Result<()> {
    // id, libelle
    try_insert(type_mission::insert(conn, 1, "Audit"))?;
    try_insert(type_mission::insert(conn, 2, "Développement"))?;
    Ok(())
}

fn seed_employees(conn: &Connection) -> Result<()> {
    // id_employee, id_category
    try_insert(employee::insert(conn, 1, 1))?; // Junior
    try_insert(employee::insert(conn, 2, 2))?; // Senior
    try_insert(employee::insert(conn, 3, 3))?; // Expert
    Ok(())
}

fn seed_intervenants(conn: &Connection) -> Result<()> {
    // id, nom, prenom, horaires, ville, cp, tel, id_planning, id_employee (Option), id_sous_traitant (Option)
    // Employee
    try_insert(intervenant::insert(conn, 1, "Martin", "Paul", "9h-17h", "Paris", "75001", "0600000001", 1, Some(1), None))?;
    // Sous-traitant
    try_insert(intervenant::insert(conn, 2, "Smith", "John", "10h-18h", "Lyon", "69001", "0600000002", 2, None, Some(1)))?;
    Ok(())
}

fn seed_missions(conn: &Connection) -> Result<()> {
    // id, temps, description, creation, debut, fin, modif, ville, dep, id_status, id_intervenant
    try_insert(mission::insert(conn, 1, "5j", "Mission Site Web", "2023-01-01", "2023-02-01", "2023-03-01", "2023-01-02", "Paris", "75", 1, 1))?;
    try_insert(mission::insert(conn, 2, "2j", "Audit Sécurité", "2023-04-01", "2023-05-01", "2023-06-01", "2023-04-02", "Lyon", "69", 2, 2))?;
    Ok(())
}

fn seed_activites(conn: &Connection) -> Result<()> {
    // id, libelle, temps, prioritaire, teletravail, id_status, id_mission
    try_insert(activite::insert(conn, 1, "Analyse Besoins", "1j", true, true, 2, 1))?;
    try_insert(activite::insert(conn, 2, "Dev Backend", "3j", true, true, 1, 1))?;
    Ok(())
}

fn seed_typer(conn: &Connection) -> Result<()> {
    // id_type, id_mission, importance
    try_insert(typer::insert(conn, 2, 1, 5))?; // Dev web -> Dev
    try_insert(typer::insert(conn, 1, 2, 3))?; // Audit -> Audit
    Ok(())
}

fn seed_associer(conn: &Connection) -> Result<()> {
    // id_client, id_mission, date
    try_insert(associer::insert(conn, 1, 1, "2023-01-01".to_string()))?;
    Ok(())
}

fn seed_posseder(conn: &Connection) -> Result<()> {
    // id_intervenant, id_competence, preference
    try_insert(posseder::insert(conn, 1, 1, 5))?; // Paul likes Rust
    try_insert(posseder::insert(conn, 1, 2, 4))?; // Paul likes SQL
    Ok(())
}

fn seed_intervenir(conn: &Connection) -> Result<()> {
    // id_intervenant, id_activite, date, heure
    try_insert(intervenir::insert(conn, 1, 1, "2023-02-10".to_string(), "10:00".to_string()))?;
    Ok(())
}

fn seed_demander(conn: &Connection) -> Result<()> {
    // id_activite, id_competence
    try_insert(demander::insert(conn, 2, 1))?; // Dev Backend requires Rust
    Ok(())
}

fn seed_occuper(conn: &Connection) -> Result<()> {
    // id_occupation, id_planning, date_debut, date_fin
    try_insert(occuper::insert(conn, 1, 1, "2023-01-10 09:00".to_string(), "2023-01-10 11:00".to_string()))?;
    Ok(())
}
