use crate::crossterm::cursor::*;
use crossterm::style::{Color, style};

use crate::GameState;
pub struct GUI {
    pub _game_state : GameState,
    pub height : u16,
    pub width : u16,
}
impl GUI {
    pub fn draw(& self){
    
        let _cursor = cursor();
        let _game = &self._game_state;

        let _player = _game.creatures.get(0)
            .expect("Game logic error: the player is dead and the game is still running.");


        //Stats
        let player_health = _player.health;  
            
        let player_name = &_player.name;

        let demage_per_hit = _player.damage;    


        //Draw stats
        _cursor.goto(0, 0);	 
        println!("{}", style(format!("Name: {}",player_name)) 
                    .with(Color::White));
        _cursor.goto(25, 0);
        println!("{}", style(format!("Demage: {}",demage_per_hit)) 
                    .with(Color::Blue));
        //Draw health
        _cursor.goto(self.width-3, 0);	
        println!("{}", style(format!("{}%", player_health)) //player health
                    .with(Color::White));

        _cursor.goto(self.width-7, 0);	    
        println!("{}", style(format!("<3: ")) //heart icon
                    .with(Color::Red));



/* -=[ goblins ]=-  6/97
             ,      ,
            /(.-""-.)\
        |\  \/      \/  /|
        | \ / =.  .= \ / |
        \( \   o\/o   / )/
         \_, '-/  \-' ,_/
           /   \__/   \
           \ \__/\__/ /
         ___\ \|--|/ /___
       /`    \      /    `\
  jgs /       '----'       \ */             //error while put in variable, ascii error (?)





        _cursor.goto(0,self.height-2);
        println!("Command:")
}
}

