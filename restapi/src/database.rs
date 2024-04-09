use rusqlite::{Connection, Result};

pub fn init() -> Result<()> {
    let conn = Connection::open("database.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            firstname TEXT NOT NULL,
            lastname TEXT NOT NULL,
            email TEXT NOT NULL,
            birthday DATE DEFAULT NULL,
            phone_1 TEXT DEFAULT NULL,
            phone_2 TEXT DEFAULT NULL,
            address TEXT DEFAULT NULL,
            city TEXT NOT NULL,
            postal_code TEXT DEFAULT NULL,
            country TEXT NOT NULL,
            biography TEXT NOT NULL,
            picture TEXT DEFAULT NULL,
            wallpaper TEXT DEFAULT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )?;

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
    Connection::open("database.db").unwrap()
}
