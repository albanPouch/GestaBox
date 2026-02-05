use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Client {
    pub id_client: i32,
    pub nom_client: String,
    pub prenom_client: String,
    pub raison_social: Option<String>,
    pub telephone_client: String,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Client(
            id_client INT,
            nom_client VARCHAR(50),
            prenom_client VARCHAR(50),
            raison_social VARCHAR(50),
            telephone_client CHAR(10),
            PRIMARY KEY(id_client)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id: i32,
    nom_client: &str,
    prenom_client: &str,
    raison_social: Option<&str>,
    telephone_client: &str,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Client (
            id_client, nom_client, prenom_client, raison_social, telephone_client
        ) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, nom_client, prenom_client, raison_social, telephone_client],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_client: i32) -> Result<Client> {
    conn.query_row(
        "SELECT id_client, nom_client, prenom_client, raison_social, telephone_client
         FROM Client WHERE id_client = ?1",
        params![id_client],
        |row| {
            Ok(Client {
                id_client: row.get(0)?,
                nom_client: row.get(1)?,
                prenom_client: row.get(2)?,
                raison_social: row.get(3)?,
                telephone_client: row.get(4)?,
            })
        },
    )
}

pub fn get_all(conn: &Connection) -> Result<Vec<Client>> {
    let mut stmt = conn.prepare("SELECT id_client, nom_client, prenom_client, raison_social, telephone_client FROM Client")?;
    let client_iter = stmt.query_map([], |row| {
        Ok(Client {
            id_client: row.get(0)?,
            nom_client: row.get(1)?,
            prenom_client: row.get(2)?,
            raison_social: row.get(3)?,
            telephone_client: row.get(4)?,
        })
    })?;

    let mut clients = Vec::new();
    for client in client_iter {
        clients.push(client?);
    }
    Ok(clients)
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}