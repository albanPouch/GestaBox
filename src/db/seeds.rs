use rusqlite::{Connection, Result};

pub const SEEDING_ENABLED: bool = true;

const SEED_SQL: &str = include_str!("seeds.sql");

pub fn seed_all(conn: &Connection) -> Result<()> {
    if !SEEDING_ENABLED {
        println!("Seeding is disabled.");
        return Ok(());
    }

    conn.execute_batch(SEED_SQL)?;
    println!("Database seeding completed successfully.");
    Ok(())
}
