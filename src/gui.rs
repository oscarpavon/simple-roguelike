use crate::crossterm::cursor::*;
use crossterm::style::{Color, style};
use std::process::Command;
use crossterm::terminal::*;
use std::str::FromStr;
use crate::GameState;
use crossterm::input;

pub struct GUI {
    pub height : u16,
    pub width : u16,
    pub center_x : u16,
    pub center_y : u16,
    pub cursor_position_x : u16,
    pub cursor_position_y : u16,
    pub show_help_screen : bool,
    pub show_message_box : bool,
    pub show_main_menu : bool,
    pub float_menu_to_draw : FloatMenu
}
pub struct FloatMenu {
    pub active : bool,
    pub selected_item : u8, //only 256 items
    pub items_string_to_draw : Vec<String>,
    pub position_x : u16,
    pub position_y : u16
}
impl FloatMenu {
    pub fn update(&mut self){
        let mut input = input();

	
        let _cursor = cursor();

        
        match input.read_char() {
            Ok(s) => {
                match s {
                    'j' => {
                        self.move_down();
                    }
                    _ => {}
                }
            }
            Err(e) => {}
        }
			
    }
       pub fn move_down(&mut self){
        self.selected_item += 1;
    }
}

pub struct draw_text {
    pub text : String
}


impl GUI {
   
    pub fn draw(& self, _game : &GameState, texts : draw_text){
       if self.show_main_menu {
           self.draw_main_menu();
       }
       else{
           self.draw_game_interface(_game,texts);
       }
      self.draw_float_menu(&self.float_menu_to_draw);
    }

    //print text only where no have GUI (min: 1 , max = height - 3 )
    //TODO: where not draw condition
    pub fn print_in_game_camera(&self, text_to_write_in_game_window : String, _color : Color, pos_x : u16 , pos_y : u16) {
        let _cursor = cursor();
        _cursor.goto(pos_x, pos_y);
        println!("{}",style(text_to_write_in_game_window).with(_color));
       
    }
    fn draw_game_interface(&self, _game : &GameState, texts : draw_text){
         let _cursor = cursor();

       self.draw_status_bar(_game);
       self.draw_enemies_names(_game);
       self.draw_weapons_list(_game);
       self.print_in_game_camera(String::from("Enemies:"), Color::Green, 0, 2);    

        _cursor.goto(15,15);
        println!("{}",texts.text);

        

       self.print_in_game_camera(String::from("Press '1' key to see help"), Color::Green, 1, self.height-4);
        if self.show_help_screen {
             let terminal = terminal();
            terminal.clear(ClearType::All);
            self.draw_help_screen();
           
        }
        if self.show_message_box {
             let terminal = terminal();
            terminal.clear(ClearType::All);
            self.draw_message_box();
        }
       
       _cursor.goto(0, self.height);//input command position
       _cursor.goto(self.cursor_position_x, self.cursor_position_y);
    }

    pub fn draw_main_menu(&self){
        for x in 0..self.width {
             self.print_in_game_camera(String::from("x"), Color::Green, x, 1);
        }
        
        let mut items_menu = Vec::new();
        let item_one = String::from("New Game");
        let item_two = String::from("Exit");
        
        items_menu.push(item_one);
        items_menu.push(item_two);    


        let mut new_float_menu = FloatMenu{
            active : true,
            selected_item : 0,
            items_string_to_draw : items_menu,
            position_x : self.center_x,
            position_y : self.center_y
        };
       
       
        self.print_in_game_camera(String::from("Simple Rusty Roguelike"), Color::Green, self.width/2-11, 4);

        //You're the only human warrior left and must defeat all enemies!
        
        

        for i in 1..self.height-3 {
             self.print_in_game_camera(String::from("x"), Color::Green, 0, i);
        }
        for i in 1..self.height-3 {
             self.print_in_game_camera(String::from("x"), Color::Green, self.width, i);
        }
        for i in 0..self.width {
             self.print_in_game_camera(String::from("x"), Color::Green, i, self.height-3);
        }
         self.draw_float_menu(&new_float_menu);
         new_float_menu.update();
         self.draw_float_menu(&new_float_menu);
    }

    pub fn draw_weapons_list(& self, _game : &GameState){
             let _cursor = cursor();
             self.print_in_game_camera(String::from("Weapons"),Color::Green,30,5);

            let weapons_list = &_game.weapon_manager.availible_weapons;
            for i in 0..weapons_list.len() {
                //self.print_in_game_camera(weapons_list[i].name,Color::Green,30,std::convert::Into<u16>(i));
                let weapon_name = &_game.weapon_manager.availible_weapons[i].name;
                let number = 6 + i;
                _cursor.goto(30, number as u16);  
                //println!("{}",weapon_name);              
                if _game.weapon_manager.availible_weapons[i].is_used{
                    println!("->{}<-",weapon_name);
                }else{
                    println!("{}",weapon_name);
                }
                
                
            }

    }
    pub fn draw_enemies_names(& self, _game : &GameState){
          
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
        println!("{}", style(format!("Damage: {}",demage_per_hit)) 
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

    pub fn draw_help_screen(& self){
        self.print_in_game_camera(String::from("- Use h j k l to move the cursor"), Color::Green, 1, self.height/2-4);
        self.print_in_game_camera(String::from("- Press 'w' to select weapon"), Color::Green, 1, self.height/2-3);
         self.print_in_game_camera(String::from("- Press 's' to select enemies"), Color::Green, 1, self.height/2-2);
          self.print_in_game_camera(String::from("- Press 'a' to atack selected enemy"), Color::Green, 1, self.height/2-1);
    }

    pub fn draw_message_box(& self) -> bool { //yes or no
        self.print_in_game_camera(String::from("Do you want to quit?"), Color::Green, 1, self.height/2-4);
        self.print_in_game_camera(String::from("yes"), Color::Green, self.width/2 - 4, self.height/2-3);
         self.print_in_game_camera(String::from("no"), Color::Green, self.width/2 + 4, self.height/2-3);
          
          true
    }

    pub fn draw_float_menu(&self, menu_to_draw : &FloatMenu){
        if menu_to_draw.active {
            for i in 0..menu_to_draw.items_string_to_draw.len(){

            let text_to_draw = &menu_to_draw.items_string_to_draw[i];
            let mut  color = Color::Red;
            if i == menu_to_draw.selected_item as usize{
                color = Color::DarkYellow;
            }
            
            self.print_in_game_camera(text_to_draw.to_string(), color, menu_to_draw.position_x, menu_to_draw.position_y + {i as u16});
            }
        }
        
        
    }

    pub fn clear(&self){
        let terminal = terminal();
        terminal.clear(ClearType::All);
    }


     pub fn create_in_another_terminal(&self) -> bool{
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