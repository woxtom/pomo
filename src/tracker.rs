use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
fn sanitize_filename(name: &str) -> String {
    // Remove invalid characters and control chars, trim trailing spaces/dots (invalid on Windows)
    let forbidden = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
    let mut s: String = name
        .trim() // remove \r\n and surrounding spaces
        .chars()
        .filter(|c| !forbidden.contains(c) && (*c as u32) >= 32)
        .collect();
    while s.ends_with(' ') || s.ends_with('.') {
        s.pop();
    }
    // Avoid Windows reserved names like CON, PRN, AUX, NUL, COM1..LPT9
    let upper = s.to_ascii_uppercase();
    let reserved = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    if reserved.contains(&upper.as_str()) {
        s.push('_');
    }
    if s.is_empty() { "_".into() } else { s }
}
fn get_project_path(project_name: &str) -> PathBuf {
    // Use proper platform-specific data directory
    let mut p_path = dirs::data_dir().expect("Could not find data directory");
    p_path.push("pomo");
    p_path.push("journal");
    std::fs::create_dir_all(&p_path).expect("Failed to create data directory");
    let project_journal_name = format!("{}.md", sanitize_filename(project_name));
    p_path.push(project_journal_name);
    p_path
}

#[derive(Debug)]
pub struct ProjectTracker {
    project_name: String,
    time_invested: f32,
    journal_path: PathBuf,
}
impl ProjectTracker {
    pub fn time_getter(&self) -> f32 {
        self.time_invested
    }
    pub fn name_getter(&self) -> &str {
        &self.project_name
    }
    pub fn path_getter(&self) -> &PathBuf {
        &self.journal_path
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
            "CREATE TABLE IF NOT EXISTS project_with_journal (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_name TEXT NOT NULL,
                time_invested FLOAT NOT NULL,
                journal_path TEXT NOT NULL
            )",
            [],
        )?;

        Ok(ProjectTrackerDb { conn })
    }
    pub fn create_project(&self, project_name: &str, time_invested: f32) -> Result<i64> {
        let journal_path = get_project_path(project_name);
        //create the journal file if it does not exist
        if !journal_path.exists() {
            std::fs::File::create(&journal_path).expect("Failed to create journal file");
        }
        self.conn.execute(
            "INSERT INTO project_with_journal (project_name, time_invested, journal_path) VALUES (?1, ?2, ?3)", 
            params![project_name.to_string(), time_invested, journal_path.to_string_lossy().to_string()]
        )?;
        Ok(self.conn.last_insert_rowid())
    }
    pub fn get_projects(&self) -> Result<Vec<ProjectTracker>> {
        let mut stmt = self.conn.prepare(
            "SELECT project_name, time_invested, journal_path FROM project_with_journal",
        )?;
        let project_iter = stmt.query_map([], |row| {
            Ok(ProjectTracker {
                project_name: row.get(0)?,
                time_invested: row.get(1)?,
                journal_path: PathBuf::from(row.get::<_, String>(2)?),
            })
        })?;

        let mut projects = Vec::new();
        for project in project_iter {
            projects.push(project?);
        }
        Ok(projects)
    }
    pub fn get_single_project(&self, project_name: &str) -> Result<Option<ProjectTracker>> {
        let mut stmt = self.conn.prepare("SELECT project_name, time_invested, journal_path FROM project_with_journal WHERE project_name = ?1")?;
        let mut rows = stmt.query(params![project_name])?;

        if let Some(row) = rows.next()? {
            Ok(Some(ProjectTracker {
                project_name: row.get(0)?,
                time_invested: row.get(1)?,
                journal_path: PathBuf::from(row.get::<_, String>(2)?),
            }))
        } else {
            Ok(None)
        }
    }
    pub fn update_project(&self, project_name: &str, time_invested: f32) -> Result<bool> {
        let updated = self.conn.execute(
            "UPDATE project_with_journal SET time_invested = ?1 WHERE project_name = ?2",
            params![time_invested, project_name],
        )?;
        Ok(updated > 0)
    }
    pub fn delete_project(&self, project_name: &str) -> Result<bool> {
        //delete the journal file associated with the project
        if let Some(project) = self.get_single_project(project_name)? {
            let journal_path = project.path_getter();
            if journal_path.exists() {
                std::fs::remove_file(journal_path).map_err(|e| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(e),
                    )
                })?;
            }
        }
        //delete the project from the database
        let deleted = self.conn.execute(
            "DELETE FROM project_with_journal WHERE project_name = ?1",
            params![project_name],
        )?;
        Ok(deleted > 0)
    }
}
