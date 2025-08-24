use std::{io, io::Write, process::exit,process::Command, thread, time::Duration};
use colored::*;

use crate::{pomodoro, tracker::ProjectTrackerDb};

// handle all cli stuff, and create visuals
pub fn welcome() {
    println!("Welcome to the {} Tracker! By {}", "Pomodoro".red().bold(), "WoftoM".green().italic());
    println!("");
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
        println!("What's your {} move?", "NEXT".bright_yellow());
        println!("{}. Start a {} session","1".yellow(),"POMODORO".red());
        println!("{}. Create a {} project","2".yellow(),"NEW".green());
        println!("{}. Delete an {} project","3".yellow(),"OLD".blue());
        println!("{}. Show your {}","4".yellow(), "STATUS".purple());
        println!("{}. {} the tracker","5".yellow(), "EXIT".cyan());
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
                Ok(_) => {println!("\r🎉 Focus session completed!                            ");success_jingle();},
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
    let ascii_art = r#"                                                                   
                                                            ____   
           .---.                   ___                    ,'  , `. 
          /. ./|          .--.,  ,--.'|_               ,-+-,.' _ | 
      .--'.  ' ;  ,---. ,--.' ;  |  | :,'   ,---.   ,-+-. ;   , || 
     /__./ \ : | '   ,'\|  | /   :  : ' :  '   ,'\ ,--.'|'   |  ;| 
 .--'.  '   \' ./   /   :  : : .;__,'  /  /   /   |   |  ,', |  ': 
/___/ \ |    ' .   ; ,. :  | |-|  |   |  .   ; ,. |   | /  | |  || 
;   \  \;      '   | |: |  : :/:__,'| :  '   | |: '   | :  | :  |, 
 \   ;  `      '   | .; |  |  .' '  : |__'   | .; ;   . |  ; |--'  
  .   \    .\  |   :    '  : '   |  | '.'|   :    |   : |  | ,     
   \   \   ' \ |\   \  /|  | |   ;  :    ;\   \  /|   : '  |/      
    :   '  |--"  `----' |  : \   |  ,   /  `----' ;   | |`-'       
     \   \ ;            |  |,'    ---`-'          |   ;/           
      '---"             `--'                      '---'            
                                                                   "#;
    println!("{}",ascii_art.bright_cyan())
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
    println!("Please {} project for your time!","SELECT".blue());
    for (id, project) in project_list.iter().enumerate() {
        println!("{}: {}", id, project.name_getter().trim());
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
    println!("How many {} would you like to focus for?", "MINUTES".purple());
    let focus_time = loop {
        let mut time_input = String::new();
        io::stdin().read_line(&mut time_input).expect("Failed to read line");
        match time_input.trim().parse::<f32>() {
            Ok(minutes) if minutes > 0.0 => {
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
    println!("");
    println!("{}ing on project '{}' for {} minutes...","FOCUS".green(), project_list[usize::from(project_index)].name_getter().trim(), focus_time);
    println!("");
    // Convert minutes to seconds
    let total_seconds = (focus_time * 60.0) as u64;
    let spinner_chars = ["⠋".red(), "⠙".magenta(), "⠹".yellow(), "⠸".green(), "⠼".cyan(), "⠴".blue(), "⠦".purple(), "⠧".black(), "⠇".bright_black(), "⠏".bright_red()];
    let mut spinner_index=0;
    for remaining in (0..total_seconds).rev() {
        let minutes = remaining / 60;
        let seconds = remaining % 60;
        for _ in 0..10 {
            
            print!("\r\x1B[2K{} Time remaining: {:02}:{:02} ", spinner_chars[spinner_index % spinner_chars.len()], minutes, seconds);
            std::io::stdout().flush().unwrap();
            
            spinner_index += 1;
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    pomodoro::focus_on_project(project_tracker_data,project_list[usize::from(project_index)].name_getter(), focus_time)
}
pub fn create_project(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String>{
    println!("What's the project's {}?","NAME".cyan());
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).expect("Failed to read line");
    //call pomodoro function
    pomodoro::create_project(project_tracker_data, project_name.as_str())
}
pub fn delete_project(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String> {
    println!("Which project do you want to {}? Please type in its full name.","DELETE".blue());
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
pub fn show_status(project_tracker_data: &ProjectTrackerDb) -> Result<bool, String> {
    println!("");
    println!("Your Focus Dashboard");
    println!("═══════════════════════════════════════");
    
    let project_list = match pomodoro::get_all_project(project_tracker_data) {
        Ok(result) => result,
        Err(e) => panic!("{}", e)
    };
    
    if project_list.is_empty() {
        return Err(format!("Wow! You haven't focused for even one minute! 😹"));
    }
    
    let total_time: f32 = project_list.iter().map(|p| p.time_getter()).sum();
    let max_name_len = project_list.iter()
        .map(|p| p.name_getter().trim().len())
        .max()
        .unwrap_or(0);
    
    for project in project_list.iter() {
        let name = project.name_getter().trim();
        let time = project.time_getter();
        let percentage = (time as f64 / total_time as f64) * 100.0;
        let bar_length = ((percentage / 100.0) * 30.0) as usize;
        
        let hours = time / 60.0;
        let minutes = time % 60.0;
        let time_str = if hours > 0.0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        };
        
        let bar = "█".repeat(bar_length) + &"░".repeat(30 - bar_length);
        
        println!(
            "{:<width$} │ {} │ {:>7} ({:5.1}%)", 
            name, bar, time_str, percentage, width = max_name_len
        );
    }
    
    println!("═══════════════════════════════════════");
    let total_hours = total_time / 60.0;
    let total_minutes = total_time % 60.0;
    println!("{} Focus Time: {}h {}m","TOTAL".cyan(), total_hours, total_minutes);
    println!("");
    Ok(true)
}
fn success_jingle() {
    let notes = [
        (523, 200), // C5 - quick
        (659, 200), // E5 - ascending  
        (784, 200), // G5 - higher
        (1047, 600), // C6 - triumphant end
    ];
    
    for (freq, duration) in notes.iter() {
        let _ = Command::new("powershell.exe")
            .args(&["-c", &format!("[Console]::Beep({}, {})", freq, duration)])
            .output();
        thread::sleep(Duration::from_millis(50));
    }
}
