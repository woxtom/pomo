use crate::cli::{action_router, action_selection};
pub mod tracker;
pub mod cli;
pub mod pomodoro;
fn main() {
    cli::welcome();
    let project_db = tracker::ProjectTrackerDb::new("/home/wot/rust/pomo/pomodoro.db").expect("Error connecting database");
    loop{
        let selected_action = action_selection();
        action_router(selected_action, &project_db);
        println!("")
    }
}
