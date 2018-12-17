mod game_state;
mod creatures;
mod commands;
mod features;
mod gui;
mod weapons;

use std::env; //for input argument

extern crate crossterm;
use crossterm::terminal::*;
use crossterm::input;
use crossterm::style::{Color, style};

use crate::features::Feature;
use crate::game_state::{GameState, PLAYER_ID};
use crate::creatures::*;
use crate::commands::*;
use crate::weapons::Weapon;
use crate::gui::*;

const GUI_DEBUG_MODE : u8 = 1; //open and run the game in new terminal
const GUI_NORMAL_MODE : u8 = 2; //run the game in the same terminal

fn main() {
	println!("Starting game..");
	
	match get_mode_number() {
		GUI_DEBUG_MODE => {
			println!("Starting in DEBUG mode");
			start_game(GUI_DEBUG_MODE);
		}
		_ => {
			println!("Starting in NORMAL mode");
			start_game(GUI_NORMAL_MODE);
		}
	}
	
}

fn start_game(mode : u8){
	let _terminal = terminal();
	
	let (_width, _height) = _terminal.terminal_size();

	let creatures =  create_creatures_structs();

	let mut state = GameState::new(creatures[0].clone()); // [0] is the player

	state.add_register(creatures[1].clone());
	state.add_register(creatures[1].clone());

	create_weapons(&mut state);
	
	let _gui = GUI {		
		height : _height,
		width : _width
		
	};

	let _input = input();

	//TODO: read input, log, event from the other terminal
	if mode == GUI_DEBUG_MODE {
		_gui.create(); 										//open the new terminal a execute the game without argument
		loop {
			
			match _input.read_line() {//this is for pause purpose
				Ok(input_command_text) => println!("string typed: {}", input_command_text), // TODO: compare with Command Struct stuff
				Err(e) => println!("error: {}", e),
			}
		}
	}else{

		_terminal.clear(ClearType::All);
		_gui.draw_main_menu();	
				
		
		//main game loop
		loop {		

			let mut text = String::new();
			let _input_command = Command::get(&state);
			
			
			match _input_command {
				Command::Attack(target) => {
					&state.hit(0, 1);
					state.hit(1,0);
				}
				Command::Examine(target) => {
					
				}
				Command::Status => {
					text = String::from("status")
				
				}
				Command::Help => {
				
				}
				Command::Debug(DebugCommand::Remove(target)) => {
				}
				Command::Dummy => {
					text = String::from("test command print =(")
				}
			}
				
			
			let mut text = draw_text {
			text : text
			};
			_terminal.clear(ClearType::All);//clear terminal before draw but produce tearing
			//input_command(&state, _input_command, &_gui);
			_gui.draw(&state,text);
			
		}
	}
	
	
}

fn get_mode_number() -> u8{
	let args : Vec<_> = env::args().collect(); //read input argument
	
	let mut imput_argument = String::from("");
	if args.len() > 1 {
		imput_argument = args[1].to_owned();
	}
	
	let _debug_string_argument = String::from("-d");

	let mut mode_number = 0;

	if imput_argument == _debug_string_argument {
		mode_number = GUI_DEBUG_MODE;
	}
	else{
		mode_number = GUI_NORMAL_MODE;
	}
	mode_number
}

fn create_creatures_structs() -> Vec<Creature> {
	let human_warrior = Creature {
		name: String::from("Thanos"),
		health: 25,
		damage: 4,
		features: vec![]
	};
	let goblin = Creature {
		name: String::from("goblin"),
		health: 12,
		damage: 2,
		features: vec![Feature::Aggression]
	};

	let mut created_creatures = Vec::new();
	created_creatures.push(human_warrior);
	created_creatures.push(goblin);	

	created_creatures 
}

fn input_command(state: &GameState, _input_command : Command, gui : &GUI){		

	match _input_command {
			Command::Attack(target) => {
				//state.hit(PLAYER_ID, target);
			}
			Command::Examine(target) => {
				let creature = state.creatures.get(target)
											  .expect("Game logic error: if the player is choosing this creature then it must exist.");
				let stylized = style(format!("{} has {} hitpoints remaining and does {} damage.",
				creature.name, creature.health, creature.damage)).with(Color::Red);
				println!("{}", stylized);
			}
			Command::Status => {
				
				//let stylized = style(format!("== There are {} enemies: {}", count.to_string(), creature_string)).with(Color::Red);
				//println!("{}", stylized);
				gui.print_in_game_camera(String::from("status"), Color::DarkBlue, 5, 5);
				//gui.text = String::from("test");
			}
			Command::Help => {
				println!("The available commands are:
attack: Hit enemies. Usage: 'attack enemy_name'
examine: Shows the status of a creature. Usage: 'examine enemy_name'
status: Show your character's status and remaining enemies."
				);
			}
			Command::Debug(DebugCommand::Remove(target)) => {
				//let creature: Creature = state.creatures.remove(target);
				//println!("Creature '{}' with the id {} has been removed from the game.", creature.name, target);
			}
			_ => {
				//
			}
		}
		
	
}

fn create_weapons(_state : &mut GameState){
	let mut big_sword = Weapon {
		name : String::from("big_sword"),
		damage : 6
	};
	let stick = Weapon {
		name : String::from("stick"),
		damage : 2
	};
	let snife = Weapon {
		name : String::from("snife"),
		damage : 4
	};

	_state.weapon_manager.add_weapon(big_sword.clone());
	_state.weapon_manager.add_weapon(stick.clone());
	_state.weapon_manager.add_weapon(snife.clone());
}