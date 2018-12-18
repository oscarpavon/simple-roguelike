use crate::crossterm::cursor::*;
use crossterm::style::{Color, style};
use std::process::Command;
use crossterm::terminal::*;
use std::str::FromStr;

use crate::GameState;
use crate::point::Point;

#[derive(PartialEq)]
pub enum GUIState {
    HelpScreen,
    MessageBox,
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
    pub fn draw(&self, _game: &GameState, text: DrawText){
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

        
        match self.state {
            GUIState::HelpScreen => {
                self.clear();
                self.draw_help_screen()
                }
                
            GUIState::MessageBox => {
                self.clear();
                self.draw_message_box()
            }
            
            GUIState::None => ()
        }
        //self.clear();

       _cursor.goto(0, self.size.y);//input command position
       _cursor.goto(self.cursor.x, self.cursor.y);//input command position
    }

    //print text only where no have GUI (min: 1 , max = height - 3 )
    //TODO: where not draw condition
    pub fn print(&self, text: DrawText) {
        let _cursor = cursor();
        _cursor.goto(text.position.x, text.position.y);
        println!("{}", style(text.text).with(text.color));
    }

    pub fn draw_main_menu(&self){
        for x in 0..self.size.x {
             self.print(DrawText::new("x").with_color(Color::Green).with_pos(x, 1));
        }

        self.print(DrawText::new("Simple Rusty Roguelike").with_color(Color::Green)
                .with_pos(self.center().x, 4));

        println!("{}", style("\n## You're the only human warrior left\n")
                    .with(Color::Green));
        println!("{}", style("\n and must defeat all enemies!\n")
                    .with(Color::Green));

        self.print(DrawText::new("--> New Game <--").with_color(Color::Green)
                .with_pos(self.center().x - 6, self.size.y - 8));
        self.print(DrawText::new("Exit").with_color(Color::Green)
                .with_pos(self.center().x - 3, self.size.y - 7));

        for i in 0..self.size.x {
            self.print(DrawText::new("x").with_color(Color::Green)
                    .with_pos(i, self.size.y - 3));
        }
        for i in 1..self.size.y - 3 {
             self.print(DrawText::new("x").with_color(Color::Green)
                    .with_pos(0, i));
             self.print(DrawText::new("x").with_color(Color::Green)
                    .with_pos(self.size.x, i));
        }
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
                .expect("Game logic error: the player is dead and the game is still running.");

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
        _cursor.goto(self.size.x - 3, 0);
        println!("{}", style(format!("{}%", player_health)) //player health
                    .with(Color::White));

        _cursor.goto(self.size.x - 7, 0);
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
