//the module acts as a state manager according to the cli commands
use crate::tracker::ProjectTrackerDb;
use crate::tracker::ProjectTracker;



pub fn delete_project(project_tracker_data: &ProjectTrackerDb, project_name: &str) -> Result<bool, String> {
    match project_tracker_data.delete_project(project_name) {
        Ok(deleted) => {
            if deleted {
                Ok(true)
            } else {
                Err(format!("Project '{}' not found.", project_name))
            }
        },
        Err(e) => Err(format!("Error deleting project: {}", e)),
    }
}
pub fn focus_on_project(project_tracker_data: &ProjectTrackerDb, project_name: &str, focus_time: f32) -> Result<bool, String> {
    let previous_focus_time = match project_tracker_data.get_single_project(project_name) {
        Ok(Some(valid_project)) => valid_project.time_getter(),
        Ok(None)=>return Err(format!("No such project named {}", project_name)),
        Err(e) => return Err(format!("Error retrieving project: {}",e)),
    };
    let time_invested = previous_focus_time+focus_time;
    match project_tracker_data.update_project(project_name, time_invested) {
        Ok(true) => Ok(true),
        Ok(false) => Err(format!("Project '{}' not found for update", project_name)),
        Err(e) => Err(format!("Error updating project: {}",e)),
    }
}
pub fn create_project(project_tracker_data: &ProjectTrackerDb, project_name: &str) -> Result<bool,String> {
    match project_tracker_data.create_project(project_name, 0.0) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Error creating project: {}", e)),
    }
}

pub fn get_all_project(project_tracker_data: &ProjectTrackerDb) -> Result<Vec<ProjectTracker>, String> {
    match project_tracker_data.get_projects() {
        Ok(result) =>  Ok(result),
        Err(e) => Err(format!("Error retrieving all files, specifically {}", e))
    }
}
