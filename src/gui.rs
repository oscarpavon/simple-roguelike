use crate::crossterm::cursor::*;
use crossterm::style::{Color, style};
use std::process::Command;
use crossterm::terminal::*;
use std::str::FromStr;
use std::io;

use crate::GameState;
use crate::point::Point;

#[derive(PartialEq)]
pub enum GUIState {
    HelpScreen,
    MessageBox,
    MainMenu,
    None
}
pub struct GUI {
    pub size: Point,
    pub state: GUIState,
    pub float_menu: FloatMenu,
    pub cursor: Point
}
pub struct FloatMenu {
    pub active: bool,
    pub selected_item: usize,
    pub item_vec: Vec<String>,
    pub position: Point
}

pub struct DrawText {
    text : String,
    position: Point,
    color: Color,
}
impl DrawText {
    pub fn new(text: &str) -> DrawText {
        DrawText {
            text: text.to_owned(),
            position: Point::new(0, 0),
            color: Color::White
        }
    }
    pub fn with_text(mut self, text: &str) -> DrawText {
        self.text = text.to_owned();
        self
    }
    pub fn with_pos(mut self, x: u16, y: u16) -> DrawText {
        self.position = Point::new(x, y);
        self
    }
    pub fn with_color(mut self, color: Color) -> DrawText {
        self.color = color;
        self
    }
}

impl GUI {
    pub fn create(&self) -> bool {
        if cfg!(target_os = "windows"){
            Command::new("cmd")
                .arg("/k")
                .arg("./target/debug/simple-roguelike.exe")
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
    pub fn draw(&mut self, _game: &mut GameState, text: DrawText){
        

       
        match self.state {
            GUIState::HelpScreen => {
                self.clear();
                self.draw_help_screen()
                }
                
            GUIState::MessageBox => {
                self.clear();
                self.draw_message_box()
            }
            GUIState::MainMenu => {
                self.clear();
                self.draw_main_menu();
                let player_name = self.get_player_name_from_input();
                _game.creatures.get_mut(0).expect("no creature").name = player_name;
                self.state = GUIState::None;
               
            }
            GUIState::None =>     {
                 self.clear();
                 self.draw_game_interface(_game,text);
            }         
            
        }
        
        //self.clear();

       cursor().goto(0, self.size.y);//input command position
       cursor().goto(self.cursor.x, self.cursor.y);//input command position
    }
     fn draw_game_interface(&self, _game : &GameState, text: DrawText){
         let _cursor = cursor();
        self.draw_status_bar(_game);
        self.draw_enemies_names(_game);
        self.draw_weapons_list(_game);
        self.print(DrawText::new("Enemies:").with_color(Color::Green).with_pos(0, 2));

        _cursor.goto(15,15);
        println!("{}", text.text);

        self.draw_float_menu(&self.float_menu);
        self.print(DrawText::new("Press '1' key to see help")
                .with_color(Color::Green).with_pos(1, self.size.y - 4));
     }

    //print text only where no have GUI (min: 1 , max = height - 3 )
    //TODO: where not draw condition
    pub fn print(&self, text: DrawText) {
        let _cursor = cursor();
        _cursor.goto(text.position.x, text.position.y);
        println!("{}", style(text.text).with(text.color));
    }

    pub fn draw_main_menu(&self){
        
        self.print(DrawText::new("Simple Rusty Roguelike").with_color(Color::Green)
                .with_pos(self.center().x-10, 4));

       // You're the only human warrior left and must defeat all enemies!
        

        self.draw_line(self.size.x, Point::new(0, 1));
       //self.draw_line(self.size.x, Point::new(0, 20));
       
        self.print(DrawText::new("nickname: ")
        .with_color(Color::Grey)       
        .with_pos(20,self.size.y - 5));
        cursor().goto(30, self.size.y - 5);
        
    }

    //horizontal line :TODO vertical 
    fn draw_line(&self, lengh : u16, initial_position : Point){
        for i in 0..lengh {
            self.print(DrawText::new("x").with_pos(initial_position.x+i, initial_position.y));
        }
    }

   pub fn get_player_name_from_input(&mut self) -> String{
        cursor().goto(self.size.x/2, self.size.y-5);
        let mut input_string_buffer = String::new();
 
        io::stdin().read_line(&mut input_string_buffer);
        let string_copy = input_string_buffer.clone();
        self.state = GUIState::None;
 
        string_copy
     }
    pub fn draw_weapons_list(& self, _game : &GameState){
             let _cursor = cursor();
             self.print(DrawText::new("Weapons").with_color(Color::Green).with_pos(30, 5));

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
        self.print(DrawText::new("Health").with_color(Color::Green).with_pos(20, 5));

        let _player = _game.creatures.get(1)
                .expect("The creature is dead and not exist");

        //Stats
        let player_health = _player.health;
        let player_name = &_player.name; //string variables need be reference

        let _cursor = cursor();

        //Draw stats
        _cursor.goto(10, 6);
        println!("{}", style(format!("{}", player_name)).with(Color::White));

        //Draw health
        _cursor.goto(20, 6);
        println!("{}", style(format!("{}%", player_health)).with(Color::White));
    }

    fn draw_status_bar(& self, _game : &GameState){
         let _cursor = cursor();

        let _player = _game.creatures.get(0)
            .expect("Game logic error: the player is dead and the game is still running.");

        //Stats
        let player_health = _player.health;
        let player_name = &_player.name;//string variables need be reference
        let damage_per_hit = _player.damage;

        //Draw stats
        _cursor.goto(0, 0);
        println!("{}", style(format!("Name: {}", player_name))
                    .with(Color::White));
        _cursor.goto(25, 0);
        println!("{}", style(format!("Damage: {}", damage_per_hit))
                    .with(Color::Blue));
        //Draw health
        _cursor.goto(self.size.x - 4, 0);
        println!("{}", style(format!("{}%", player_health)) //player health
                    .with(Color::White));

        _cursor.goto(self.size.x - 8, 0);
        println!("{}", style(format!("<3: ")) //heart icon
                    .with(Color::Red));

        _cursor.goto(0, self.size.y - 2);
        println!("Command:")
    }

    pub fn draw_help_screen(& self){
        self.print(DrawText::new("- Use h j k l to move the cursor").with_color(Color::Green)
                .with_pos(1, self.center().y - 4));
        self.print(DrawText::new("- Press 'w' to select weapon").with_color(Color::Green)
                .with_pos(1, self.center().y - 3));
        self.print(DrawText::new("- Press 's' to select enemies").with_color(Color::Green)
                .with_pos(1, self.center().y - 2));
        self.print(DrawText::new("- Press 'a' to atack selected enemy").with_color(Color::Green)
                .with_pos(1, self.center().y - 1));
        self.print(DrawText::new("- Press ':' and the write a command").with_color(Color::Green)
                .with_pos(1, self.center().y - 1));

       
        self.print(DrawText::new("The available commands are:
        attack: Hit enemies. Usage: 'attack enemy_name'
        examine: Shows the status of a creature. Usage: 'examine enemy_name'
        status: Show your character's status and remaining enemies.").with_color(Color::Green)
                .with_pos(1, self.center().y + 3));
    }

    pub fn draw_message_box(&self) {
        self.print(DrawText::new("Do you want to quit?").with_color(Color::Green)
                .with_pos(1, self.center().y - 4));
        self.print(DrawText::new("yes").with_color(Color::Green)
                .with_pos(self.center().x - 4, self.center().y - 3));
        self.print(DrawText::new("no").with_color(Color::Red)
                .with_pos(self.center().x + 4, self.center().y - 3));
    }

    pub fn draw_float_menu(&self, menu: &FloatMenu){
        if menu.active {
            for i in 0..menu.item_vec.len(){
                let text = &menu.item_vec[i];

                let color = if i == menu.selected_item {
                    Color::DarkYellow
                } else {
                    Color::Red
                };

                self.print(DrawText::new(text).with_color(color)
                        .with_pos(menu.position.x, menu.position.y + {i as u16}));
            }
        }
    }
    pub fn clear(&self){
        let terminal = terminal();
        terminal.clear(ClearType::All);
    }
    fn center(&self) -> Point {
        self.size / 2
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
