use crate::crossterm::cursor::*;
use crate::crossterm::terminal::*;
use crossterm::style::{Color, style};

pub struct GUI {
    pub height : u32,
    pub width : u32
}
impl GUI {
    pub fn draw(& self){
    

    
    let _terminal = terminal();
    //_terminal.clear(ClearType::All);

    let _cursor = cursor();
	_cursor.hide();
    _cursor.save_position();	
	
	let (width, height) = _terminal.terminal_size();	
	_cursor.goto(width - 10, 0);	//always in the right corner (width - char count)
	println!("{}", style(format!("Health: {}", 2))
				   .with(Color::Red));	

	//_cursor.reset_position(); //back to the original position for writen the other text
}
}
