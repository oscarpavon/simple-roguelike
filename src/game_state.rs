use crossterm::style::{Color, style};

use crate::creatures::*;
use crate::features::{Feature, aggressive_system, player_system};

use crate::weapons::*;

pub const PLAYER_ID: CreatureId = 0;

pub struct Debug{
    pub logs : Vec<String>
}
impl Debug {
    pub fn print_all_log(&self){
        for i in 0..self.logs.len() {
            println!("{}",self.logs[i]);
        }
    }
    pub fn log(&mut self, text : String){
        self.logs.push(text);
    }
}
#[derive(Clone)]
pub struct GameInput {
	pub mode : u8,// mode 1 = read key ; mode 2 = read line
	pub key : char
}

pub struct GameState {
	pub creatures: CreatureMap,
	pub aggressive: Vec<CreatureId>,
	pub weapon_manager : WeaponManager,
	pub input : GameInput,
	pub debug : Debug
	
}

// just used for determining console output
enum AttackDirection {
	ToPlayer,
	Neutral,
	FromPlayer
}
impl AttackDirection {
	fn to_color(&self) -> Color {
		match self {
			ToPlayer => Color::Red,
			Neutral => Color::White,
			FromPlayer => Color::Green
		}
	}
}

impl GameState {
	pub fn new(player: Creature) -> GameState {
		let new_input_data= GameInput {
			mode : 0,
			key : ':'
		};
		let mut new_debug = Debug{
            logs : Vec::new()
        };
        new_debug.log(String::from("log01"));
		let mut state = GameState {
			creatures: CreatureMap::new(),
			aggressive: Vec::new(),
			weapon_manager: WeaponManager::new(),
			input : new_input_data,
			debug : new_debug
		};
		state.creatures.add(player);
		state
	}
	pub fn add_register(&mut self, creature: Creature) -> CreatureId {
		let id = self.creatures.len();

		for feature in &creature.features {
			match feature {
				Feature::Aggression => self.aggressive.push(id)
			}
		}

		self.creatures.add(creature)
	}
	#[allow(dead_code)]
	pub fn remove_feature(&mut self, id: CreatureId, feature: Feature) {
		let creature = self.creatures.get_mut(id)
									 .expect("Game logic error: can't remove if feature if creature doesn't exist.");
		if let Some(feature_index) = creature.features.iter().position(|x| *x == feature) {
			creature.features.remove(feature_index);
		}
	}
	pub fn round(&mut self) -> bool {
		// systems.
		player_system(self);
		aggressive_system(self);

		true // TODO: player_system can return this, if not then the game will close because of the player's will
	}
	// Hits a creature with the inflictor's name and damage.
	pub fn hit(&mut self, inflictor_id: CreatureId, target_id: CreatureId) {

		assert!(inflictor_id != target_id, "Game logic error: a creature can't attack itself.");

		// get name and damage from inflictor
		let (name, damage) = {
			// Can use unwrap because the target the inflictor is hitting must exist
			let inflictor = self.creatures.get(inflictor_id)
										  .expect("Game logic error: the inflictor must exist, in order to call this function.");
			(inflictor.name.clone(), inflictor.damage)
		};
		// get name and apply damage to target
		let (target_name, target_health) = {
			// Can unwrap because the target must be alive.
			let target = self.creatures.get_mut(target_id)
									   .expect("Game logic error: the target must exist, in order to be hit.");
			target.health -= damage;
			(target.name.clone(), target.health)
		};
		// english stuff
		let mut direction = AttackDirection::Neutral;

		let inflictor_str = if inflictor_id == PLAYER_ID {
			direction = AttackDirection::FromPlayer;
			"+ You hit".to_owned()
		} else {
			format!("{} hit", name)
		};
		let target_str = if target_id == PLAYER_ID {
				direction = AttackDirection::ToPlayer;
				"you".to_owned()
			} else {
				target_name
			};
		let final_str = format!("{} {} for {} damage.", inflictor_str, target_str, damage.to_string());

		println!("{}", style(final_str)
					   .with(direction.to_color()));

		if target_health > 0 {
			if target_id != PLAYER_ID {
				let final_str = format!("> {} now has {} hitpoints remaining.", target_str, target_health.to_string());
				println!("{}", style(final_str).with(Color::Green));
			}
		} else {
			self.die(target_id);
		}
	}
	pub fn die(&mut self, dead_id: CreatureId) {
		let creature = self.creatures.remove(dead_id);

		let error_str = "Game internal error: creature with feature is not on its respective list.";
		for feature in creature.features {
			match feature {
				Feature::Aggression => self.aggressive.remove(self.aggressive.iter()
																			 .position(|x| *x == dead_id)
																			 .expect(error_str))
			};
		}

		let target_str = if dead_id == PLAYER_ID {
						 	 "You died!".to_owned()
						 } else {
						 	 format!("{} has died!", creature.name)
						 };

		println!("{}", target_str);
	}

	pub fn get_alive_creatures_name(&self) -> Vec<String> {
		let mut creatures_list_name = Vec::new();

		let mut creature_string = String::new();

		let mut count = 0usize;
		// Can unwrap because alive() ASSURES that the returned creatures are alive.
		for creature in self.creatures.alive().iter()
											.filter(|id| **id != PLAYER_ID)
											.map(|id| self.creatures.get(*id)
											.expect("Game internal error: alive() function returned a None.")) {
			creature_string.push_str(creature.name.as_str());
			creatures_list_name.push(creature_string.clone());
			creature_string = String::new();
			
			//creature_string.push_str(
			//	format!("{}; \n ", creature.name).as_str()
			//);
			count += 1;
		}

		/* if count == 0 {
			println!("=============== You WIN! ==============");
		} else {
			let stylized = style(format!("== There are {} enemies: {}", count.to_string(), creatures_string)).with(Color::Red);
			//println!("{}", stylized);
		} */
		creatures_list_name
	}

}
