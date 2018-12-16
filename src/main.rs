mod game_state;
mod creatures;
mod commands;
mod features;
mod gui;

use std::env; //for input argument
extern crate crossterm;
use crossterm::terminal::*;
use crossterm::input;
use crossterm::style::{Color, style};

use crate::features::Feature;
use crate::game_state::GameState;
use crate::creatures::*;

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

	let creatures =  create_struct_creatures();

	let mut state = GameState::new(creatures[0].clone()); // [0] is the player

	state.add_register(creatures[1].clone());
	state.add_register(creatures[1].clone());

	
	
	let _gui = GUI {		
		height : _height,
		width : _width		
	};

	if mode == GUI_DEBUG_MODE {
		_gui.create();
		loop {
			let _input = input();
			match _input.read_line() {//this is for pause purpose
				Ok(input_command_text) => println!("string typed: {}", input_command_text), // TODO: compare with Command Struct stuff
				Err(e) => println!("error: {}", e),
			}
		}
	}else{

		_terminal.clear(ClearType::All);
		_gui.draw_main_menu();
		let _input = input();
		
		
		//loop
		loop {
			//playing..
			//input()
			
			match _input.read_line() {//this is for pause purpose
				Ok(input_command_text) => println!("string typed: {}", input_command_text), // TODO: compare with Command Struct stuff
				Err(e) => println!("error: {}", e),
			}
			//system_player()
			_terminal.clear(ClearType::All);//clear terminal before draw but produce tearing
			
			_gui.draw(&state);
			
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

fn create_struct_creatures() -> Vec<Creature> {
	let human_warrior = Creature {
		name: String::from("human_warrior"),
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
