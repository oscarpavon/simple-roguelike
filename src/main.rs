mod game_state;
mod creatures;
mod commands;
mod features;
mod gui;
mod weapons;
mod point;

use std::env; //for input argument
use std::process;
use std::io;
use std::fs::File;
use std::io::prelude::*;

extern crate crossterm;
use crossterm::terminal::*;
use crossterm::input;

use crate::crossterm::cursor::*;
use crossterm::style::{Color, style};

use crate::features::Feature;
use crate::game_state::{GameState, PLAYER_ID};
use crate::creatures::*;
use crate::commands::*;
use crate::weapons::Weapon;
use crate::gui::*;
use crate::point::Point;

const GUI_DEBUG_MODE : u8 = 1; //open and run the game in new terminal
const GUI_NORMAL_MODE : u8 = 2; //run the game in the same terminal

fn main() {
	println!("Starting game..");

	start_game();

}

fn start_game () {
	match get_game_start_mode_number() {
			GUI_DEBUG_MODE => {
				println!("Starting in DEBUG mode");
				init_data_go_in_loop_game(GUI_DEBUG_MODE);
			}
			_ => {
				println!("Starting in NORMAL mode");
				init_data_go_in_loop_game(GUI_NORMAL_MODE);
			}
		}
}
//
fn init_data_go_in_loop_game(mode : u8){

	let _terminal = terminal();

	let (_width, _height) = _terminal.terminal_size();

	let creatures =  create_creatures_structs();

	let mut state = GameState::new(creatures[0].clone()); // [0] is the player

	state.add_register(creatures[1].clone());
	state.add_register(creatures[2].clone());

	let mut items_menu = Vec::new();
	let item_one = String::from("Attack");
	let item_two = String::from("Examine");
	let item_three = String::from("Use");
	items_menu.push(item_one);
	items_menu.push(item_two);
	items_menu.push(item_three);

	let mut menus = Vec::new();
	let creatures_names = state.get_alive_creatures_name();

    let mut new_float_menu = FloatMenu::new(Point::new(0, 15))
                            .with_array_items(creatures_names.clone());	

	

	
	create_weapons(&mut state);
	let attack_float_menu = FloatMenu{
		focus : true,
		visible: false,
		selected_item: 0,
		item_vec: items_menu.clone(),
		position: Point::new(45,8),
	};

	let other_menu = FloatMenu{
		focus : false,
		visible: false,
		selected_item: 0,
		item_vec: items_menu,
		position: Point::new(0,8),
	};

	menus.push(attack_float_menu);
	let mut _gui = GUI {
		size: Point::new(_width, _height),
		state: GUIState::MainMenu,
		float_menu: other_menu,
		cursor: Point::empty(),
		menus_to_draw : menus,
		active_menu_number : 0		
	};

	
	
	match mode {
		GUI_NORMAL_MODE => {
			main_game_loop(&mut state, &mut _gui);
		}
		GUI_DEBUG_MODE => {//game exected with -d argument
			terminal_debug_loop(&mut _gui);
		}
		_ => println!("{}", "ERROR: No game mode available")
	}
	

}

fn main_game_loop(state : &mut GameState, _gui : &mut GUI){
	_gui.clear();
	//main game loop
	loop {
		let (_width, _height) = terminal().terminal_size();//update console size
		_gui.size = Point::new(_width, _height);
		//_gui.draw(state, DrawText::new("test"));//first time draw main menu and get input name
		//_gui.draw(state, DrawText::new("test"));//draw the game interface and pause
		input_control(state, _gui);
		
		//input_command(&state, _input_command, &_gui);
		_gui.draw(state, DrawText::new("test"));//draw game interface with update data
		
	}
}

fn create_creatures_structs() -> Vec<Creature> {
	let human_warrior = Creature {
		name: String::from("Thanos"),
		health: 100,
		damage: 4,
		features: vec![]
	};
	let goblin = Creature {
		name: String::from("goblin"),
		health: 12,
		damage: 2,
		features: vec![Feature::Aggression]
	};
	let brown_goblin = Creature {
		name: String::from("brown_goblin"),
		health: 12,
		damage: 2,
		features: vec![Feature::Aggression]
	};

	let mut created_creatures = Vec::new();
	created_creatures.push(human_warrior);
	created_creatures.push(goblin);
	created_creatures.push(brown_goblin);
	

	created_creatures
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

fn input_control(state : &mut GameState , gui : &mut GUI) {
	let _cursor = cursor();

	match input().read_char() {
		Ok(s) => {
			state.input.key = s; //assing input to game state
			match s {
				'k' => {					
					if gui.cursor.y > 0{
						gui.cursor.y -= 1		//implement in GUI   		gui.cursor_move_down()
					}				
				}
				'j' => {
					if gui.cursor.y < gui.size.y {
					gui.cursor.y += 1
					}

				}
				'h' => {
					if gui.cursor.x > 0 {
						gui.cursor.x -= 1
					}					
				}
				'l' => gui.cursor.x += 1,
				'e' => {//enemies select position
					gui.cursor.x = 12;
					gui.cursor.y = 6;
				}
				'w' => {//weapons select position
					gui.cursor.x = 30;
					gui.cursor.y = 6;
				}
				'f' => {//focus next menu
					gui.focus_next_menu();
				}
				's' => {//select
					//gui.cursor.x = 12;
					//gui.cursor.y = 6;
					/* gui.float_menu.visible = if gui.float_menu.visible == false {
						gui.float_menu.focus = true;
						true
					} else {
						gui.float_menu.focus = false;
						false
					} */
					gui.menus_to_draw[0].visible = true;
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
				'm' => {//main menu
					//back to the game with the same game
					gui.state = if gui.state != GUIState::MainMenu {
						GUIState::MainMenu
					} else {
						GUIState::None
					};
				}
				':' => {//command mode
 
  						cursor().goto(0, gui.size.y);
 						print!("{}",':');
 						cursor().goto(1, gui.size.y);
 						let mut input_string_buffer = String::new();
 
  						io::stdin().read_line(&mut input_string_buffer);
						let command = Command::get(state, input_string_buffer, gui);
 						input_command(state, command, gui);
				}
				'c' => {//console mode
						gui.state = GUIState::ConsoleMode;
						gui.clear();
				}
				_ => {}
			}
		}
		Err(e) => println!("char error : {}", e)
	}
}

fn input_command(state: &mut GameState, _input_command : Command, gui : &mut GUI){

	match _input_command {
			Command::Attack(target) => {
				//state.hit(PLAYER_ID, target);
				let creatures_names = state.get_alive_creatures_name();

				let mut new_float_menu = FloatMenu::new(Point::new(50, 2))
									.with_array_items(creatures_names);      
			
				gui.menus_to_draw.push(new_float_menu);
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
				
				//gui.text = String::from("test");

			}
			Command::Help => {
				//show gui help
				
			}
			Command::Debug(DebugCommand::Remove(target)) => {
				state.creatures.remove(target);
				//println!("Creature '{}' with the id {} has been removed from the game.", creature.name, target);
			}
			Command::Exit => {
 				
				process::exit(0x0100); //on linux but 0x0256 on Windows :TODO
 
  			}
			Command::OpenFile => {
				open_file();
			}
			Command::Save => {
				//save name in file
				save_file();
			}
			Command::NewCreature => {
				let goblin = Creature {
					name: String::from("goblin5"),
					health: 12,
					damage: 2,
					features: vec![Feature::Aggression]
				};
				state.add_register(goblin.clone());

			}
			_ => {
				//
			}
		}



}

fn terminal_debug_loop(_gui : &mut GUI) {
	_gui.create_in_new_terminal(); 										//open the new terminal and execute the game without argument
	loop {
		//TODO: read input, log, event from the other terminal
		match input().read_line() {//this is for pause purpose
			Ok(input_command_text) => println!("string typed: {}", input_command_text), // TODO: compare with Command Struct stuff
			Err(e) => println!("error: {}", e),
		}
	}
}

fn get_game_start_mode_number() -> u8{
	let args : Vec<_> = env::args().collect(); //read input argument

	let mut input_argument = String::from("");
	if args.len() > 1 {
		input_argument = args[1].to_owned();
	}

	let _debug_open_new_console_string_argument = String::from("-d");	

	let mut mode_number = 0;
	if input_argument == _debug_open_new_console_string_argument {
		mode_number = GUI_DEBUG_MODE;
	}else{
		mode_number = GUI_NORMAL_MODE;
	}
	
	mode_number
}
fn save_file() -> std::io::Result<()>{
	let mut file = File::create("./src/save_data.txt")?;
  	 file.write_all(b"player name here")?;
   	Ok(())
}
	
fn open_file() -> std::io::Result<()> {
    let mut file = File::open("./src/save_data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    //assert_eq!(contents, "Hello, world!");
    Ok(())
}

/* 			println!( */