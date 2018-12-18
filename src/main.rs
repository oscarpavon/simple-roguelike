mod game_state;
mod creatures;
mod commands;
mod features;
mod gui;
mod weapons;
mod point;

use std::env; //for input argument

extern crate crossterm;
use crossterm::terminal::*;
use crossterm::input;

use crate::crossterm::cursor::*;
use crossterm::style::{Color, style};
use crate::input::*;
use std::process;

use crate::features::Feature;
use crate::game_state::{GameState, PLAYER_ID};
use crate::creatures::*;
use crate::commands::*;
use crate::weapons::Weapon;
use crate::gui::*;
use crate::point::Point;

const GUI_DEBUG_MODE : u8 = 1; //open and run the game in new terminal
const GUI_NORMAL_MODE : u8 = 2; //run the game in the same terminal
const GUI_DISABLED_MODE : u8 = 3; //run the game in legacy mode

fn main() {
	println!("Starting game..");

	match get_game_start_mode_number() {
		GUI_DEBUG_MODE => {
			println!("Starting in DEBUG mode");
			start_game(GUI_DEBUG_MODE);
		}
		GUI_DISABLED_MODE => {
			println!("Starting in game command mode");
			start_game(GUI_DISABLED_MODE);
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

	let mut items_menu = Vec::new();
	let item_one = String::from("Attack");
	let item_two = String::from("Examine");
	let item_three = String::from("Use");
	items_menu.push(item_one);
	items_menu.push(item_two);
	items_menu.push(item_three);

	create_weapons(&mut state);
	let new_float_menu = FloatMenu{
		active: false,
		selected_item: 0,
		item_vec: items_menu,
		position: Point::new(45,8),
	};

	let mut _gui = GUI {
		size: Point::new(_width, _height),
		state: GUIState::None,
		float_menu: new_float_menu,
		cursor: Point::empty(),
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
	} else {
		_gui.clear();
		_gui.draw_main_menu();

		//main game loop
		loop {
			let (_width, _height) = _terminal.terminal_size();//update console size
			_gui.size = Point::new(_width, _height);

			input_control(&mut _gui);
			_gui.clear();
			//input_command(&state, _input_command, &_gui);
			_gui.draw(&state, DrawText::new("test"));
		}
	}


}

fn get_game_start_mode_number() -> u8{
	let args : Vec<_> = env::args().collect(); //read input argument

	let mut imput_argument = String::from("");
	if args.len() > 1 {
		imput_argument = args[1].to_owned();
	}

	let _debug_open_new_console_string_argument = String::from("-d");
	let _debug_game_console_gui_disabled_string_argument = String::from("-c");

	let mut mode_number = 0;
	match imput_argument {
		_debug_game_console_gui_disabled_string_argument => {
			mode_number = GUI_DISABLED_MODE
		}
		_debug_open_new_console_string_argument => {
			mode_number = GUI_DEBUG_MODE;
		}
		_ => mode_number = GUI_NORMAL_MODE

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
				gui.print(DrawText::new("status").with_color(Color::DarkBlue).with_pos(5, 5));
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
	let big_sword = Weapon {
		name : String::from("big_sword"),
		damage : 6,
		is_used : true
	};
	let stick = Weapon {
		name : String::from("stick"),
		damage : 2,
		is_used : false
	};
	let snife = Weapon {
		name : String::from("snife"),
		damage : 4,
		is_used : false
	};

	_state.weapon_manager.add_weapon(big_sword.clone());
	_state.weapon_manager.add_weapon(stick.clone());
	_state.weapon_manager.add_weapon(snife.clone());
}

fn input_control(gui : &mut GUI) {
	let _cursor = cursor();

	match input().read_char() {
		Ok(s) => {
			match s {
				'k' => {
					if gui.float_menu.active == true {
						//if gui.float_menu.selected_item == gui.float_menu.items_string_to_draw.len() as u8{

							gui.float_menu.selected_item -= 1;

					} else {
						gui.cursor.y -= 1
					}

				}
				'j' => {
					if gui.float_menu.active == true {
						gui.float_menu.selected_item += 1;
					}else{
						gui.cursor.y += 1
					}

				}
				'h' => gui.cursor.x -= 1,
				'l' => gui.cursor.x += 1,
				'e' => {//enemies select position
					gui.cursor.x = 12;
					gui.cursor.y = 6;
				}
				'w' => {//weapons select position
					gui.cursor.x = 30;
					gui.cursor.y = 6;
				}
				's' => {//select
					gui.cursor.x = 12;
					gui.cursor.y = 6;
					gui.float_menu.active = if gui.float_menu.active == false {
						true
					} else {
						false
					}

				}
				'1' => {
					gui.state = if gui.state != GUIState::HelpScreen {
						GUIState::HelpScreen
					} else {
						GUIState::None
					};
				}
				'q' => {
					gui.state = if gui.state != GUIState::MessageBox {
						GUIState::MessageBox
					} else {
						GUIState::None
					};
				}
				'y' => {//yes
					match gui.state {
						GUIState::MessageBox => {
							gui.clear();
							process::exit(0x0100);
						}
						_ => ()
					}
				}
				'n' => {
					match gui.state {
						GUIState::MessageBox => gui.state = GUIState::None,
						_ => ()
					}
				}
				_ => {}
			}
		}
		Err(e) => println!("char error : {}", e)
	}
}
