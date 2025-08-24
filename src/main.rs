use crate::cli::{action_router, action_selection};
use std::path::PathBuf;
pub mod tracker;
pub mod cli;
pub mod pomodoro;

fn get_database_path() -> PathBuf {
    // Use proper platform-specific data directory
    let mut db_path = dirs::data_dir().expect("Could not find data directory");
    db_path.push("pomo");
    std::fs::create_dir_all(&db_path).expect("Failed to create data directory");
    db_path.push("pomodoro.db");
    db_path
}

fn main() {
    cli::welcome();
    let db_path = get_database_path();
    let project_db = tracker::ProjectTrackerDb::new(db_path.to_str().unwrap()).expect("Error connecting database");
    loop{
        let selected_action = action_selection();
        action_router(selected_action, &project_db);
        println!("")
    }
}
