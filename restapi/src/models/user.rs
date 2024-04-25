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

    pub fn get(&self, id: i64) -> Option<UserOUT> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM users WHERE id = ?1")
            .unwrap();
        let user_iter = stmt
            .query_map(&[&id], |row| {
                Ok(UserOUT {
                    id: row.get(0)?,
                    firstname: row.get(1)?,
                    lastname: row.get(2)?,
                    email: row.get(3)?,
                    phone_1: row.get(4)?,
                    phone_2: row.get(5)?,
                    address: row.get(6)?,
                    city: row.get(7)?,
                    country: row.get(8)?,
                    biography: row.get(9)?,
                    picture: row.get(10)?,
                    wallpaper: row.get(11)?,
                })
            })
            .unwrap();
        user_iter
            .map(|user| user.unwrap())
            .next()
    }
}
