use std::{io, io::Write, process::exit,process::Command, thread, time::Duration};
use colored::*;
use crate::pomodoro;
use crate::tracker::ProjectTrackerDb;

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
    WriteJounal,
    Exit,
}
pub fn action_selection() -> Action {
    loop {
        println!("What's your {} move?", "NEXT".bright_yellow());
        println!("{}. Start a {} session","1".yellow(),"POMODORO".red());
        println!("{}. Create a {} project","2".yellow(),"NEW".green());
        println!("{}. Delete an {} project","3".yellow(),"OLD".blue());
        println!("{}. Show your {}","4".yellow(), "STATUS".bright_magenta());
        println!("{}. Project {}","5".yellow(),"JOURNALING".bright_black());
        println!("{}. {} the tracker","6".yellow(), "EXIT".cyan());
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read line");
        // convert selection to Action
        match selection.trim().parse::<u8>() {
            Ok(1) => return Action::Pomodoro,
            Ok(2) => return Action::CreateNewProject,
            Ok(3) => return Action::DeleteProject,
            Ok(4) => return Action::ShowStatus,
            Ok(5) => return Action::WriteJounal,
            Ok(6) => return Action::Exit,
            Ok(7) => return Action::EasterEgg,
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
                Ok(true) => {println!("\r🎉 Focus session completed!                            ");success_jingle();},
                Ok(false) => {
                    println!("");
                }
                Err(e) => {println!("{}",e);},
            } 
        },
        Action::CreateNewProject => {
            match create_project(project_tracker_data) {
                Ok(true) => {println!("Project created! ✅")}
                Ok(false) => {
                    println!("");
                }
                Err(e) => {println!("{}",e)}
            };
        },
        Action::EasterEgg => {
            // Easter egg action
            easter_egg();
        },
        Action::DeleteProject => {
            match delete_project(project_tracker_data) {
                Ok(true) => {
                    println!("Project deleted! ✅")
                }
                Ok(false) => {
                    println!("");
                }
                Err(e) => {println!("{}",e)}
            };
        },
        Action::ShowStatus => {
            match show_status(project_tracker_data) {
                Ok(_) => {println!("You're cool! Do you know what you need to do to become an expert is only to focus on one thing for 10,000 hours? 🍻 CHEERS!");},
                Err(e) => {println!("{}",e);},
            };
        },
        Action::WriteJounal => {
            match journal_mode(project_tracker_data) {
                Ok(true) => {
                    println!("Keep Journaling! It's a good habit!");
                }
                Ok(false) => {
                    println!("");
                }
                Err(e) => {
                    println!("{}",e);
                }
            }
        },
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
    println!("Please {} project for your time! (type 'cancel' to abort)","SELECT".blue());
    for (id, project) in project_list.iter().enumerate() {
        println!("{}: {}", id, project.name_getter().trim());
    }
    let project_index = loop{
        let mut project_selection = String::new();
        io::stdin().read_line(&mut project_selection).expect("Failed to read line");
        
        // Check for cancellation
        if project_selection.trim().eq_ignore_ascii_case("cancel") {
            println!("Focus session cancelled! 🚫");
            return Ok(false);
        }
        
        match project_selection.trim().parse::<u8>() {
            Ok(index) if usize::from(index) < project_list.len() => {
                break index;
            }
            Ok(_) => {
                println!("Invalid project index. Please try again.");
            }
            Err(_) => {
                println!("Please type in your desired project's id or 'cancel' to abort.");
            }
        }
    };
    println!("How many {} would you like to focus for? (type 'cancel' to abort)", "MINUTES".purple());
    let focus_time = loop {
        let mut time_input = String::new();
        io::stdin().read_line(&mut time_input).expect("Failed to read line");
        
        // Check for cancellation
        if time_input.trim().eq_ignore_ascii_case("cancel") {
            println!("Focus session cancelled! 🚫");
            return Ok(false);
        }
        
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
    println!("Press ENTER to start (or type 'cancel' to abort): ");
    
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input).expect("Failed to read line");
    
    // Check for cancellation
    if start_input.trim().eq_ignore_ascii_case("cancel") {
        println!("Focus session cancelled! 🚫");
        return Ok(false);
    }
    
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
    pomodoro::focus_on_project(project_tracker_data,project_list[usize::from(project_index)].name_getter().trim(), focus_time)
}
pub fn create_project(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String>{
    println!("What's the project's {}, or type 'cancel' to abort: ","NAME".cyan());
    let mut project_name = String::new();
    io::stdin().read_line(&mut project_name).expect("Failed to read line");
    let project_name= project_name.trim();
    if project_name.is_empty() {
        return Err(format!("Project name cannot be empty.").into());
    }
    if project_name.eq_ignore_ascii_case("cancel") {
        println!("Project creation cancelled! 🚫");
        return Ok(false);
    }
    pomodoro::create_project(project_tracker_data, project_name)
}
pub fn delete_project(project_tracker_data:&ProjectTrackerDb) -> Result<bool, String> {
    println!("Which project do you want to {}? Please type in its full name (or 'cancel' to abort).","DELETE".blue());
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
    
    // Check for cancellation
    if project_name.trim().eq_ignore_ascii_case("cancel") {
        println!("Deletion cancelled! 🚫");
        return Ok(false);
    }
    
    pomodoro::delete_project(project_tracker_data, project_name.trim())
}
pub fn show_status(project_tracker_data: &ProjectTrackerDb) -> Result<bool, String> {
    println!("");
    println!("Your Focus Dashboard");
    println!("═══════════════════════════════════════");
    
    let project_list = match pomodoro::get_all_project(project_tracker_data) {
        Ok(result) => result,
        Err(e) => {
            return Err(e);
        }
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
        let time_str = format!("{}h", hours);
        let bar = "█".repeat(bar_length) + &"░".repeat(30 - bar_length);
        
        println!(
            "{:<width$} │ {} │ {:>7} ({:5.1}%)", 
            name, bar, time_str, percentage, width = max_name_len
        );
    }
    
    println!("═══════════════════════════════════════");
    let total_hours = total_time / 60.0;
    println!("{} Focus Time: {}h","TOTAL".cyan(), total_hours);
    println!("");
    Ok(true)
}
pub fn journal_mode(project_tracker_data: &ProjectTrackerDb) -> Result<bool, String> {
    let project_list = match pomodoro::get_all_project(project_tracker_data) {
        Ok(result) =>result,
        Err(e)=>{
            panic!("{}",e)
        }
    };
    if project_list.is_empty() {
        return Err(format!("No journal to write. You may create a new one."));
    }
    println!("Please {} project for journaling! (type 'cancel' to abort)","SELECT".blue());
    for (id, project) in project_list.iter().enumerate() {
        println!("{}: {}", id, project.name_getter().trim());
    }
    let project_index = loop{
        let mut project_selection = String::new();
        io::stdin().read_line(&mut project_selection).expect("Failed to read line");
        
        // Check for cancellation
        if project_selection.trim().eq_ignore_ascii_case("cancel") {
            println!("Journaling cancelled! 🚫");
            return Ok(false);
        }
        
        match project_selection.trim().parse::<u8>() {
            Ok(index) if usize::from(index) < project_list.len() => {
                break index;
            }
            Ok(_) => {
                println!("Invalid project index. Please try again.");
            }
            Err(_) => {
                println!("Please type in your desired project's id or 'cancel' to abort.");
            }
        }
    };
    pomodoro::write_journal(project_tracker_data, project_list[usize::from(project_index)].name_getter().trim())
}
fn success_jingle() {
    let notes = [
        (523, 200), // C5 - quick
        (659, 200), // E5 - ascending  
        (784, 200), // G5 - higher
        (1047, 600), // C6 - triumphant end
    ];
    
    if std::env::consts::OS == "windows" {
        // Windows: use PowerShell for proper tunes
        for (freq, duration) in notes.iter() {
            let _ = Command::new("powershell.exe")
                .args(&["-c", &format!("[Console]::Beep({}, {})", freq, duration)])
                .output();
            thread::sleep(Duration::from_millis(50));
        }
    } else {
        // Linux: use ASCII bell character
        for _ in 0..4 {
            print!("\x07");
            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    }
}
