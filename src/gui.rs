use crate::crossterm::cursor::*;
use crate::crossterm::terminal::*;
use crossterm::style::{Color, style};

pub fn draw(){
    let mut player_health = 0;

    if player_health < 2000 {
        player_health += 1;
    }
    
    
    let _terminal = terminal();
    //_terminal.clear(ClearType::All);

    let _cursor = cursor();
	_cursor.hide();
    _cursor.save_position();	
	
	let (width, height) = _terminal.terminal_size();	
	_cursor.goto(width - 10, 0);	//always in the right corner (width - char count)
	println!("{}", style(format!("Health: {}", player_health))
				   .with(Color::Red));	

	//_cursor.reset_position(); //back to the original position for writen the other text
}