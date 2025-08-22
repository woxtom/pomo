use rusqlite::{Connection, Result, params};

#[derive(Debug)]
pub struct ProjectTracker {
    project_name: String,
    time_invested: f32,
}
impl ProjectTracker {
    pub fn time_getter(&self) -> f32 {
        self.time_invested
    }
    pub fn name_getter(&self) -> &str {
        &self.project_name
    }
}

pub struct ProjectTrackerDb {
    conn: Connection,
}

impl ProjectTrackerDb {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        
        // Create table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS project (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_name TEXT NOT NULL,
                time_invested FLOAT NOT NULL
            )",
            [],
        )?;

        Ok(ProjectTrackerDb { conn })
    }
    pub fn create_project(&self, project_name: &str, time_invested: f32) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO project (project_name, time_invested) VALUES (?1, ?2)", 
            params![project_name.to_string(), time_invested]
        )?;
        Ok(self.conn.last_insert_rowid())
    }
    pub fn get_projects(&self) -> Result<Vec<ProjectTracker>> {
        let mut stmt = self.conn.prepare("SELECT project_name, time_invested FROM project")?;
        let project_iter = stmt.query_map([], |row| {
            Ok(ProjectTracker {
                project_name: row.get(0)?,
                time_invested: row.get(1)?,
            })
        })?;

        let mut projects = Vec::new();
        for project in project_iter {
            projects.push(project?);
        }
        Ok(projects)
    }
    pub fn get_single_project(&self, project_name: &str) -> Result<Option<ProjectTracker>> {
        let mut stmt = self.conn.prepare("SELECT project_name, time_invested FROM project WHERE project_name = ?1")?;
        let mut rows = stmt.query(params![project_name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(ProjectTracker {
                project_name: row.get(0)?,
                time_invested: row.get(1)?,
            }))
        } else {
            Ok(None)
        }
    }
    pub fn update_project(&self, project_name: &str, time_invested: f32) -> Result<bool> {
        let updated = self.conn.execute(
            "UPDATE project SET time_invested = ?1 WHERE project_name = ?2",
            params![time_invested, project_name]
        )?;
        Ok(updated>0)
    }
    pub fn delete_project(&self, project_name: &str) -> Result<bool> {
        let deleted = self.conn.execute(
            "DELETE FROM project WHERE project_name = ?1",
            params![project_name]
        )?;
        Ok(deleted > 0)
    }
}