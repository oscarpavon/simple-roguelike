use crossterm::style::{Color, style};

use crate::game_state::{GameState, PLAYER_ID};
use crate::creatures::Creature;
use crate::commands::*;

use crate::crossterm::cursor::*;
use crate::crossterm::terminal::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Feature {
	Aggression
}

pub fn aggressive_system(state: &mut GameState) {
	for i in 0..state.aggressive.len() {
		state.hit(state.aggressive[i], PLAYER_ID);
	}
}

pub fn player_system(state: &mut GameState) {
	// Can unwrap here because the player should exist.
	// If not then why should the game even be running.
	let player_health = state.creatures.get(PLAYER_ID)
									   .expect("Game logic error: the player is dead and the game is still running.")
									   .health;
	
	
	// Player control consists of three phases:
	// 1- Show the enviroment and conditions:	
	

	let mut creature_string = String::new();

	let mut count = 0usize;
	// Can unwrap because alive() ASSURES that the returned creatures are alive.
	for creature in state.creatures.alive().iter()
										   .filter(|id| **id != PLAYER_ID)
										   .map(|id| state.creatures.get(*id)
										   .expect("Game internal error: alive() function returned a None.")) {
		creature_string.push_str(
			format!("{}; ", creature.name).as_str()
		);
		count += 1;
	}

	if count == 0 {
		println!("=============== You WIN! ==============");
	} else {
		let stylized = style(format!("== There are {} enemies: {}", count.to_string(), creature_string)).with(Color::Red);
		println!("{}", stylized);
	}

	
}
