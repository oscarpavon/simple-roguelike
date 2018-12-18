use crossterm::style::{Color, style};

use crate::game_state::{GameState, PLAYER_ID};
use crate::creatures::CreatureId;

const DEBUG_MODE_ENABLED: bool = true;

pub enum Command {
	Attack(CreatureId),
	Examine(CreatureId),
	Status,
    Help,
    Debug(DebugCommand),
    Dummy,
    Exit,
    Save,
    OpenFile

}
pub enum DebugCommand {
    Remove(CreatureId)
}

impl Command {
   
	pub fn get(state: &GameState, input_string_command : String) -> Command {
		
           
			let parts: Vec<&str> = input_string_command.trim().split(' ').collect();

            // The repetition of parts.len() > 1 is acknowledged but is necessary due to one-worded
            // commands, such as 'status' or 'help'
             println!("{}",parts[0]);
			match parts[0] {
               
				"attack" => {
					if parts.len() > 1 {
						if let Some(target) = state.creatures.find(parts[1]) {
                            if target != PLAYER_ID {
                                return Command::Attack(target);
                            } else {
                                println!("{}", style("Don't attack yourself!")
                                            .with(Color::DarkRed))
                            }
						}
					}
                    println!("{}", style("Please write a correct target: ex: 'attack goblin'.")
                                   .with(Color::DarkRed))
				}
				"examine" => {
					if parts.len() > 1 {
						if let Some(target) = state.creatures.find(parts[1]) {
							Command::Examine(target);
						}
					}
                    println!("{}", style("Please write a correct target: ex: 'examine goblin'.")
                                   .with(Color::DarkRed))
				}
				"status" => {                   
					return Command::Status;                    
				}
				"help" => {
    				return Command::Help;
				}
                "exit" => {
                    return Command::Exit;
                }
                "debug" => {
                    if DEBUG_MODE_ENABLED {
                        if parts.len() > 1 {
        					match parts[1] {
                                "remove" => {
                                    if parts.len() > 2 {
                                        if let Some(target) = state.creatures.find(parts[2]) {
                        				    Command::Debug(DebugCommand::Remove(target));
                					    }
                                    }
                                }
                                _ => println!("{}",
                                              style(format!("'{}' is not a correct debug command.", parts[1]))
                                              .with(Color::DarkRed))
                            }
        				}
                        println!("{}", style("Please write an existing debug command: 'debug remove goblin'.")
                                       .with(Color::DarkRed))
                    } else {
                        println!("{}", style("Debug mode is disabled.")
                                   .with(Color::DarkRed))
                    }
                }
                "save" => {
                    return Command::Save;
                }

                "openfile" => {
                    return Command::OpenFile;
                }
                _ => {
                    println!("{}",
                              style(format!("'{}' is not a correct command.", parts[0]))
                              .with(Color::DarkRed))
                            
                }
			}

			//input_string_command.clear(); //clean when out of the scope ? read the manual =)
            Command::Dummy //for test
		
	}
}
