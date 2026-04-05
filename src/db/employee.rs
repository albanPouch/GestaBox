use rusqlite::{params, Connection, Result};

// =======================
// Structure (Données)
// =======================
#[derive(Debug, Clone)]
pub struct Employee {
    pub id_employee: i32,
    pub id_category: i32,
}

// =======================
// Création de la table
// =======================
pub fn init_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Employee (
            id_employee INTEGER PRIMARY KEY,
            id_category INTEGER,
            FOREIGN KEY (id_category) REFERENCES CategorieEmploye (id_categorie)
        )",
        [],
    )?;
    Ok(())
}

// =======================
// Insertion
// =======================
pub fn insert(
    conn: &Connection,
    id_employee: i32,
    id_category: i32,
) -> Result<()> {
    conn.execute(
        "INSERT INTO Employee (id_employee, id_category)
         VALUES (?1, ?2)",
        params![id_employee, id_category],
    )?;
    Ok(())
}

pub fn update(conn: &Connection, id_employee: i32, id_category: i32) -> Result<()> {
    conn.execute(
        "UPDATE Employee
         SET id_category = ?2
         WHERE id_employee = ?1",
        params![id_employee, id_category],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection, id_employee: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM Employee WHERE id_employee = ?1",
        params![id_employee],
    )?;
    Ok(())
}

// =======================
// Lecture par ID
// =======================
pub fn get_by_id(conn: &Connection, id: i32) -> Result<Employee> {
    conn.query_row(
        "SELECT id_employee, id_category
         FROM Employee
         WHERE id_employee = ?1",
        params![id],
        |row| {
            Ok(Employee {
                id_employee: row.get(0)?,
                id_category: row.get(1)?,
            })
        },
    )
}

// =======================
// Initialisation DB
// =======================
pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(conn)?;
    Ok(())
}

// =======================
// Lecture de tous les employés
// =======================
pub fn get_all(conn: &Connection) -> Result<Vec<Employee>> {
    let mut stmt = conn.prepare(
        "SELECT id_employee, id_category FROM Employee"
    )?;

    let employee_iter = stmt.query_map([], |row| {
        Ok(Employee {
            id_employee: row.get(0)?,
            id_category: row.get(1)?,
        })
    })?;

    let mut employees = Vec::new();
    for employee in employee_iter {
        employees.push(employee?);
    }

    Ok(employees)
}
