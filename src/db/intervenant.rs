use rusqlite::{params, Connection, Result};

// La Structure (Données)
#[derive(Debug)]
pub struct Intervenant {
    pub id_intervant: i32,
    pub nom_intervenant: String,
    pub prenom_intervenant: String,
    pub horaires: String,
    pub ville: String,
    pub code_postal: String,
    pub telephone_intervant: String,
    pub id_planning: i32,
    pub id_employee: i32,
    pub id_sous_traitant: i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Intervenant(
            id_intervant INT,
            nom_intervenant VARCHAR(50),
            prenom_intervenant VARCHAR(50),
            horaires VARCHAR(50),
            ville VARCHAR(50),
            code_postal CHAR(10),
            telephone_intervant CHAR(10),
            id_planning INT,
            id_employee INT,
            id_sous_traitant INT,
            FOREIGN KEY(id_planning) REFERENCES Planning(id_planning),
            FOREIGN KEY(id_employee) REFERENCES  Employee (id_employee),
            FOREIGN KEY(id_sous_traitant) REFERENCES SousTraitant (id_sous_traitant),
            PRIMARY KEY(id_intervant)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_intervant: i32,
    nom_intervenant: &str,
    prenom_intervenant: &str,
    horaires: &str,
    ville: &str,
    code_postal : &str,
    telephone_intervant: &str,
    id_planning : i32,
    id_employee: Option<i32>,
    id_sous_traitant: Option<i32>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Intervenant (
            id_intervant, nom_intervenant, prenom_intervenant, horaires, ville, code_postal, telephone_intervant , id_planning, id_employee, id_sous_traitant
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6 , ?7, ?8, ?9, ?10)",
        params![id_intervant, nom_intervenant, prenom_intervenant,
            horaires, ville, code_postal ,telephone_intervant, id_planning, id_employee ,id_sous_traitant ],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id_intervant: i32) -> Result<Intervenant> {
    conn.query_row(
        "SELECT id_intervant, nom_intervenant, prenom_intervenant,
         horaires, ville , code_postal, telephone_intervant, id_planning, id_employee, id_sous_traitant
         FROM Intervenant WHERE id_intervant = ?1",
        params![id_intervant],
        |row| {
            Ok(Intervenant {
                id_intervant: row.get(0)?,
                nom_intervenant: row.get(1)?,
                prenom_intervenant: row.get(2)?,
                horaires: row.get(3)?,
                ville: row.get(4)?,
                code_postal: row.get(5)?,
                telephone_intervant:row.get(6)?,
                id_planning : row.get(7)?,
                id_employee: row.get(8)?,
                id_sous_traitant: row.get(9)?,
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}