use rusqlite::{params, Connection, Result};
use crate::db::get_connection;

// La Structure (Données)
#[derive(Debug)]
pub struct Employee {
    pub id_employee: i32,
    pub id_category: i32,
}

// Création de la table
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Employee(
            id_employee INT,
            id_category INT,
            FOREIGN KEY(id_category)  REFERENCES CategorieEmploye (id_categorie),
            PRIMARY KEY(id_employee)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert(
    conn: &Connection,
    id_employee: i32,
    id_category: i32,

) -> Result<()> {
    conn.execute(
        "INSERT INTO Employee (
            id_employee, id_category
        ) VALUES (?1, ?2)",
        params![id_employee, id_category],
    )?;
    Ok(())
}

// 4. Lecture
pub fn get_by_id(conn: &Connection, id: i32) -> Result<Employee> {
    conn.query_row(
        "SELECT id_employee, id_category
         FROM Employee WHERE id_employee = ?1",
        params![id],
        |row| {
            Ok(Employee {
                id_employee: row.get(0)?,
                id_category: row.get(1)?
            })
        },
    )
}

// 5. On initialise la table et ajoute des données par défaut
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(&conn)?;
    Ok(())
}
