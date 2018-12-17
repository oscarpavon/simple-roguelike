use crate::crossterm::cursor::*;
use crossterm::style::{Color, style};
use std::process::Command;
use std::str::FromStr;
use crate::GameState;
pub struct GUI {
    pub height : u16,
    pub width : u16,
       
}
pub struct draw_text {
    pub text : String
}


impl GUI {
    pub fn create(&self) -> bool{
        if cfg!(target_os = "windows"){
            Command::new("cmd")
                .arg("/k")
                .arg("./target/debug/simple-roguelike")
                .output()
                .expect("failed to open cmd");
        }
        else{
            Command::new("urxvt")//urxvt is my terminal
                    .arg("-e")
                    .arg("./target/debug/simple-roguelike")                     
                    .output()
                    .expect("failed to execute process");
        }         
        true
    }
    pub fn draw(& self, _game : &GameState, texts : draw_text){
        let _cursor = cursor();

       self.draw_status_bar(_game);
       self.draw_enemies_names(_game);
       self.print_in_game_camera(String::from("Enemies:"), Color::Green, 0, 2);    

        _cursor.goto(15,15);
        println!("{}",texts.text);

       self.print_in_game_camera(String::from("Type 'help' to see the available commands."), Color::Green, 1, self.height-4);

       
       _cursor.goto(0, self.height);//input command position
    }

    //print text only where no have GUI (min: 1 , max = height - 3 )
    //TODO: where not draw condition
    pub fn print_in_game_camera(&self, text_to_write_in_game_window : String, _color : Color, pos_x : u16 , pos_y : u16) {
        let _cursor = cursor();
        _cursor.goto(pos_x, pos_y);
        println!("{}",style(text_to_write_in_game_window).with(_color));//TODO: add color features
       
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

    pub fn draw_enemies_names(& self, _game : &GameState){
            //TODO
             let _cursor = cursor();
            self.print_in_game_camera(String::from("Health"),Color::Green,20,5);

            let _player = _game.creatures.get(1)
            .expect("Game logic error: the player is dead and the game is still running.");


            //Stats
            let player_health = _player.health;  
                
            let player_name = &_player.name;//string variables need be reference

            let demage_per_hit = _player.damage;   

                //Draw stats
            _cursor.goto(10, 6);	 
            println!("{}", style(format!("{}",player_name)) 
                        .with(Color::White));
            //Draw health
            _cursor.goto(20, 6);	
            println!("{}", style(format!("{}%", player_health)) //player health
                        .with(Color::White));

    }

    fn draw_status_bar(& self, _game : &GameState){
         let _cursor = cursor();

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

        _cursor.goto(0,self.height-2);
        println!("Command:")
    }
}

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