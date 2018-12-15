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




        self.print_in_game_camera(String::from("Type 'help' to see the available commands."), Color::Green, 1, self.height-4);
        _cursor.goto(0,self.height-2);
        println!("Command:")
    }
    //print text only where no have GUI (min: 1 , max = height - 3 )
    //TODO: where not draw condition
    pub fn print_in_game_camera(&self, text_to_write_in_game_window : String, _color : Color, pos_x : u16 , pos_y : u16) {
        let _cursor = cursor();
        _cursor.goto(pos_x, pos_y);
        println!("{}",text_to_write_in_game_window);//TODO: add color features
    }


    pub fn draw_main_menu(&self){
        for x in 0..self.width {
             self.print_in_game_camera(String::from("x"), Color::Green, x, 1);
        }
        
        
        self.print_in_game_camera(String::from("Simple Rusty Roguelike"), Color::Green, self.width/2-11, 4);

        println!("{}", style("\n## You're the only human warrior left\n")
                    .with(Color::Green));
        println!("{}", style("\n and must defeat all enemies!\n")
                    .with(Color::Green));

        self.print_in_game_camera(String::from("-->Play<--"), Color::Green, self.width/2-6, self.height - 8);//concept
        self.print_in_game_camera(String::from("Exit"), Color::Green, self.width/2-3, self.height - 7);

        for i in 1..self.height-3 {
             self.print_in_game_camera(String::from("x"), Color::Green, 0, i);
        }
        for i in 1..self.height-3 {
             self.print_in_game_camera(String::from("x"), Color::Green, self.width, i);
        }
        for i in 0..self.width {
             self.print_in_game_camera(String::from("x"), Color::Green, i, self.height-3);
        }
    }
}

