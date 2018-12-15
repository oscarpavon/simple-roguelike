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

	let line = style("##########################################").with(Color::DarkYellow);
	println!("{}", line);
	println!("{}", style("######### Simple Rusty Roguelike #########").with(Color::DarkYellow));
	println!("{}", line);


	println!("{}", style("\n## You're the only human warrior left and must defeat all enemies!\n")
				   .with(Color::Green));

	println!("{}", style("Type 'help' to see the available commands.")
				   .with(Color::DarkGreen));

	println!("{}", style("Press Enter key to play")
				   .with(Color::White));

	state.add_register(goblin.clone());
	state.add_register(goblin.clone());

	
	
	let _gui = GUI {
		_game_state : state,		
		height : _height,
		width : _width		
	};

	let _input = input();
	//loop
	while true {
		//playing
		match _input.read_line() {
     		Ok(s) => println!("string typed: {}", s),
     		Err(e) => println!("error: {}", e),
 		}
		_terminal.clear(ClearType::All);
		_gui.draw();
		
		//input()
		//system_player()
	}

}

