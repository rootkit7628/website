use rusqlite::Connection;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    id: Option<i32>,
    name: String,
    description: String,
}

pub struct ProjectORM {
    conn: Connection
}

impl ProjectORM {
    pub fn new(conn: Connection) -> Self {
        Self { conn: conn }
    }

    pub fn get_all(&self) -> Vec<Project> {
        let mut stmt = self.conn.prepare("SELECT * FROM projects").unwrap();
        let projects = stmt.query_map([], |row| {
            Ok(Project {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?
            })
        });
        projects.unwrap().map(|p| p.unwrap()).collect()
    }

    pub fn insert(&self, project: Project) {
        self.conn.execute(
            "INSERT INTO projects (name, description) VALUES (?1, ?2)",
            &[&project.name, &project.description]
        ).unwrap();
    }
}