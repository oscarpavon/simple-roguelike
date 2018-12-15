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
            
        let player_name = &_player.name;//string variables need be reference

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
  jgs /       '----'       \ */             //error while put this character in a variable, ascii error (?)




        
        _cursor.goto(0,self.height-2);
        println!("Command:")
    }
    //print text only where no have GUI (min: 1 , max = height - 3 )
    pub fn print_in_game_camera(&self, text_to_write_in_game_window : String, _color : Color, pos_x : u16 , pos_y : u16) {
        let _cursor = cursor();
        _cursor.goto(pos_x, pos_y);
        println!("{}",text_to_write_in_game_window);
    }


    pub fn DrawMainMenu(&self){
        for x in 0..self.width {
             self.print_in_game_camera(String::from("x"), Color::Green, x, 1);
        }
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

    }
}

