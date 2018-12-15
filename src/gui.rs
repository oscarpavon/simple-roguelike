use crate::crossterm::cursor::*;
use crate::crossterm::terminal::*;
use crate::crossterm::input;
use crossterm::style::{Color, style};

use crate::GameState;
pub struct GUI {
    pub _game_state : GameState,
    pub height : u16,
    pub width : u16
}
impl GUI {
    pub fn draw(& self){
    
    let _game = &self._game_state;
   // let player_health = self._game_state.creatures.get(0).health;
     let player_health = _game.creatures.get(0)
        .expect("Game logic error: the player is dead and the game is still running.")
	    .health;   

    let _cursor = cursor();					
      
	//_cursor.hide();
    //_cursor.save_position();	
	
	_cursor.goto(self.width-2, 0);	
	println!("{}", style(format!("{}", player_health))
				   .with(Color::White));		
	_cursor.goto(self.width-5, 0);	    
	println!("{}", style(format!("<3: "))
				   .with(Color::Red));	
    
    
    

	//_cursor.reset_position(); //back to the original position for writen the other text
    
    _cursor.goto(0,self.height-2);
    println!("Command:")
}
}
