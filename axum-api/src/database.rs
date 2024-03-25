use rusqlite::{Connection, Result};

pub fn init() -> Result<()> {
    let conn = Connection::open("database.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;
    conn.close().unwrap();
    Ok(())
}

pub fn connect() -> Connection {
    let conn = Connection::open("database.db").unwrap();
    conn
}
