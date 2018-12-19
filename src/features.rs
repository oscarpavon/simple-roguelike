use crate::game_state::{GameState, PLAYER_ID};


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
	
	//let player_health = state.creatures.get(PLAYER_ID)
	//								   .expect("Game logic error: the player is dead and the game is still running.")
	//								   .health;		
}
