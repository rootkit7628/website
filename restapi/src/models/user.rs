use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserOUT {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone_1: String,
    pub phone_2: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub biography: String,
    pub picture: String,
    pub wallpaper: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserIN {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub phone_1: String,
    pub phone_2: String,
    pub address: String,
    pub city: String,
    pub country: String,
    pub biography: String,
    pub picture: String,
    pub wallpaper: String,
}

pub struct UserORM {
    conn: Connection,
}

impl UserORM {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn create(&self, user: &UserIN) -> i64 {
        self.conn
            .execute(
                "INSERT INTO users (firstname, lastname, email, phone_1, phone_2, address, city, country, biography, picture, wallpaper) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                [&user.firstname, &user.lastname, &user.email, &user.phone_1, &user.phone_2, &user.address, &user.city, &user.country, &user.biography, &user.picture, &user.wallpaper],
            )
            .unwrap();
        self.conn.last_insert_rowid()
    }
}
