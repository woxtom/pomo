use std::{io, io::Write, process::exit};

use crate::{pomodoro, tracker::ProjectTrackerDb};

// handle all cli stuff, and create visuals
pub fn welcome() {
    println!("Welcome to the Pomodoro Tracker!  Delivered by WoftoM  🍅");
}
pub enum Action {
    Pomodoro,
    CreateNewProject,
    DeleteProject,
    EasterEgg,
    ShowStatus,
    Exit,
}
pub fn action_selection() -> Action {
    loop {
        println!("What's your next move? 🤗");
        println!("1. Start a Pomodoro session");
        println!("2. Create a new project");
        println!("3. Delete an Old project");
        println!("4. Show your focus status");
        println!("5. Exit");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read line");
        // convert selection to Action
        match selection.trim().parse::<u8>() {
            Ok(1) => return Action::Pomodoro,
            Ok(2) => return Action::CreateNewProject,
            Ok(3) => return Action::DeleteProject,
            Ok(4) => return Action::ShowStatus,
            Ok(5) => return Action::Exit,
            Ok(6) => return Action::EasterEgg,
            _ => {
                println!("Invalid selection. Please try again.");
                continue;
            }
        }
    }
}
pub fn action_router(action: Action, project_tracker_data:&ProjectTrackerDb) {
    match action {
        Action::Pomodoro => {
            // Start a Pomodoro session
            match focus_mode(project_tracker_data) {
                Ok(_) => {println!("\r🎉 Focus session completed!                            ");},
                Err(e) => {println!("{}",e);},
            } 
        },
        Action::CreateNewProject => {
            match create_project(project_tracker_data) {
                Ok(_) => {println!("Project created! ✅")}
                Err(e) => {println!("{}",e)}
            };
        },
        Action::EasterEgg => {
            // Easter egg action
            easter_egg();
        },
        Action::DeleteProject => {
            match delete_project(project_tracker_data) {
                Ok(_) => {println!("Project deleted! ✅")}
                Err(e) => {println!("{}",e)}
            };
        },
        Action::ShowStatus => {
            match show_status(project_tracker_data) {
                Ok(_) => {println!("You're cool! Do you know what you need to do to become an expert is only to focus on one thing for 10,000 hours? 🍻 CHEERS!");},
                Err(e) => {println!("{}",e);},
            };
        }
        Action::Exit => {
            println!("Goodbye! 🍅");
            exit(0);
        },
    };
}
pub fn easter_egg() {
    println!("Thanks for using the Pomodoro Tracker! 🍅");
    println!(r#"                                                                   
                                                            ____   
           .---.                   ___                    ,'  , `. 
          /. ./|          .--.,  ,--.'|_               ,-+-,.' _ | 
      .--'.  ' ;  ,---. ,--.'  \ |  | :,'   ,---.   ,-+-. ;   , || 
     /__./ \ : | '   ,'\|  | /\/ :  : ' :  '   ,'\ ,--.'|'   |  ;| 
 .--'.  '   \' ./   /   :  : : .;__,'  /  /   /   |   |  ,', |  ': 
/___/ \ |    ' .   ; ,. :  | |-|  |   |  .   ; ,. |   | /  | |  || 
;   \  \;      '   | |: |  : :/:__,'| :  '   | |: '   | :  | :  |, 
 \   ;  `      '   | .; |  |  .' '  : |__'   | .; ;   . |  ; |--'  
  .   \    .\  |   :    '  : '   |  | '.'|   :    |   : |  | ,     
   \   \   ' \ |\   \  /|  | |   ;  :    ;\   \  /|   : '  |/      
    :   '  |--"  `----' |  : \   |  ,   /  `----' ;   | |`-'       
     \   \ ;            |  |,'    ---`-'          |   ;/           
      '---"             `--'                      '---'            
                                                                   "#);
}
pub fn focus_mode(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String>{
    let project_list = match pomodoro::get_all_project(project_tracker_data) {
        Ok(result) =>result,
        Err(e)=>{
            panic!("{}",e)
        }
    };
    if project_list.is_empty() {
        return Err(format!("No Project to focus. You may create a new one."));
    }
    println!("Please select project for your time!");
    for (id, project) in project_list.iter().enumerate() {
        println!("{}: {}", id, project.name_getter());
    }
    let project_index = loop{
        let mut project_selection = String::new();
        io::stdin().read_line(&mut project_selection).expect("Failed to read line");
        match project_selection.trim().parse::<u8>() {
            Ok(index) if usize::from(index) < project_list.len() => {
                break index;
            }
            Ok(_) => {
                println!("Invalid project index. Please try again.");
            }
            Err(_) => {
                println!("Please type in your desired project's id.");
            }
        }
    };
    println!("How many minutes would you like to focus for?");
    let focus_time = loop {
        let mut time_input = String::new();
        io::stdin().read_line(&mut time_input).expect("Failed to read line");
        match time_input.trim().parse::<u32>() {
            Ok(minutes) if minutes > 0 => {
                break minutes as f32;
            }
            Ok(_) => {
                println!("Please enter a positive number of minutes.");
            }
            Err(_) => {
                println!("Please enter a valid number of minutes.");
            }
        }
    };
    println!("Focusing on project '{}' for {} minutes...", project_list[usize::from(project_index)].name_getter(), focus_time);
    
    // Convert minutes to seconds
    let total_seconds = (focus_time * 60.0) as u64;
    let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    
    for remaining in (0..total_seconds).rev() {
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        let spinner = spinner_chars[(remaining % 10) as usize];
        
        // Clear the current line and print the timer
        print!("\r\x1B[2K{} Time remaining: {:02}:{:02} ", spinner, minutes, seconds);
        std::io::stdout().flush().unwrap();
        
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    pomodoro::focus_on_project(project_tracker_data,project_list[usize::from(project_index)].name_getter(), focus_time)
}
pub fn create_project(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String>{
    println!("What's the project's name? 🤔");
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).expect("Failed to read line");
    //call pomodoro function
    pomodoro::create_project(project_tracker_data, project_name.as_str())
}
pub fn delete_project(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String> {
    println!("Which project do you want to delete? Please type in its full name.");
    let project_list = match pomodoro::get_all_project(project_tracker_data) {
        Ok(result) =>result,
        Err(e)=>{
            panic!("{}",e)
        }
    };
    if project_list.is_empty() {
        return Err(format!("No Project to delete."));
    };
    for project in project_list.iter(){
        println!("{}: {} minutes", project.name_getter().trim(), project.time_getter());
    };
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).expect("Failed to read line");
    pomodoro::delete_project(project_tracker_data, project_name.as_str())
}
pub fn show_status(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String> {
    println!("Your previous focus status is shown as below🤓: ");
    let project_list = match pomodoro::get_all_project(project_tracker_data) {
        Ok(result) =>result,
        Err(e)=>{
            panic!("{}",e)
        }
    };
    if project_list.is_empty() {
        return Err(format!("Wow! You haven't focused for even one minute! 😹"));
    };
    for project in project_list.iter() {
        println!("{}: {} minutes", project.name_getter().trim(), project.time_getter());
    }
    Ok(true)
}