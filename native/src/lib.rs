mod api;
mod bridge_generated;
mod envelopes;
use std::path::Path;

use rusqlite::Connection;

pub fn create_database_instance<P: AsRef<Path>>(database_path: P) -> anyhow::Result<()> {
    let conn = Connection::open(database_path)?;
    migrations(conn)
}

fn migrations(conn: Connection) -> anyhow::Result<()> {
    migration_1(&conn)?;
    Ok(())
}

fn migration_1(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch("
        BEING
        CREATE TABLE IF NOT EXISTS envelopes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            allocated_amount REAL NOT NULL
        );
        COMMIT;",
    )?;
    Ok(())
}
