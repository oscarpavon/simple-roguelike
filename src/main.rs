mod game_state;
mod creatures;
mod commands;
mod features;
mod gui;

extern crate crossterm;
use crossterm::terminal::*;
use crossterm::input;
use crossterm::style::{Color, style};

use crate::features::Feature;
use crate::game_state::GameState;
use crate::creatures::*;

use crate::gui::*;

fn main() {

	let _terminal = terminal();
	_terminal.clear(ClearType::All);
	let (_width, _height) = _terminal.terminal_size();

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
	let mut state = GameState::new(human_warrior.clone());

	
	state.add_register(goblin.clone());
	state.add_register(goblin.clone());

	
	
	let _gui = GUI {
		_game_state : state,		
		height : _height,
		width : _width		
	};

	_gui.draw_main_menu();
	let _input = input();
	//loop
	while true {
		//playing..
		//input()
		match _input.read_line() {//this is for pause purpose
     		Ok(input_command_text) => println!("string typed: {}", input_command_text), // TODO: compare with Command Struct stuff
     		Err(e) => println!("error: {}", e),
 		}
		//system_player()
		_terminal.clear(ClearType::All);//clear terminal before draw but produce tearing
		_gui.draw();
		
		
		
	}

}

