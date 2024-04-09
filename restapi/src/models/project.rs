use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectOUT {
    pub id: i64,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectIN {
    pub name: String,
    pub description: String,
}

pub struct ProjectORM {
    conn: Connection,
}

impl ProjectORM {
    pub fn new(conn: Connection) -> Self {
        Self { conn }
    }

    pub fn get_all(&self) -> Vec<ProjectOUT> {
        let mut stmt = self.conn.prepare("SELECT * FROM projects").unwrap();
        let projects = stmt.query_map([], |row| {
            Ok(ProjectOUT {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        });
        projects.unwrap().map(|p| p.unwrap()).collect()
    }

    pub fn get(&self, id: i64) -> Option<ProjectOUT> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM projects WHERE id = ?1")
            .unwrap();
        let res = stmt.query_row([&id], |row| {
            Ok(ProjectOUT {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        });
        res.ok()
    }

    pub fn create(&self, project: &ProjectIN) -> i64 {
        self.conn
            .execute(
                "INSERT INTO projects (name, description) VALUES (?1, ?2)",
                [&project.name, &project.description],
            )
            .unwrap();
        self.conn.last_insert_rowid()
    }

    pub fn update(&self, id: i64, data: &ProjectIN) -> ProjectOUT {
        match self.get(id) {
            Some(mut project) => {
                project.name = data.name.clone();
                project.description = data.description.clone();
                self.conn
                    .execute(
                        "UPDATE projects SET name = ?1, description = ?2 WHERE id = ?3",
                        [&project.name, &project.description, &id.to_string()],
                    )
                    .unwrap();
                project
            }
            None => {
                let id = self.create(data);
                ProjectOUT {
                    id,
                    name: data.name.clone(),
                    description: data.description.clone(),
                }
            }
        }
    }

    pub fn delete(&self, id: i64) {
        self.conn
            .execute("DELETE FROM projects WHERE id = ?1", [&id])
            .unwrap();
    }
}
