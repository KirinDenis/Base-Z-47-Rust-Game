
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use console::Term;
use console::Style;
use console::Color;

//use crate::levels::get_current_level;

use crate::levels::CLEVEL;

pub fn draw() {



    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level") {


    let term = Term::stdout();
    let style: Style = Style::new();

//    let mut x = 0;
//    let mut y = 0;

//    for row in level.iter(){
//      for &cell in row.iter() {
       for y in (0..19) {
        for x in (0..19) {

         let cell = level[y][x];	
         if cell == '#' //Wall
         {
           term.move_cursor_to(x, y).unwrap(); 
           print!("{}", style.apply_to("\u{2588}\u{2588}\u{2588}\u{2588}"));           
           term.move_cursor_to(x, y+1).unwrap(); 
           print!("{}", style.apply_to("\u{2588}\u{2588}\u{2588}\u{2588}"));           
         }
 	else 
         if cell == '.' //Base
         {
           term.move_cursor_to(x, y).unwrap(); 
           print!("{}", style.apply_to(" \u{250C}\u{2500}\u{2510} "));           
           term.move_cursor_to(x, y+1).unwrap(); 
           print!("{}", style.apply_to(" \u{2514}\u{2500}\u{2518} "));           
         }
 	else 
         if cell == '$' //Box
         {
           term.move_cursor_to(x, y).unwrap(); 
           print!("{}", style.apply_to(" \u{2554}\u{2550}\u{2557} "));           
           term.move_cursor_to(x, y+1).unwrap(); 
           print!("{}", style.apply_to(" \u{255A}\u{2550}\u{255D} "));           
         }         
 	else 
         if cell == '@' //Hero
         {
           term.move_cursor_to(x, y).unwrap(); 
           print!("{}", style.apply_to("@@@@ "));           
           term.move_cursor_to(x, y+1).unwrap(); 
           print!("{}", style.apply_to("@@@@"));           
         }         
         else  //Default space 
	 {
           term.move_cursor_to(x, y).unwrap(); 
           print!("{}", style.apply_to("     "));           
           term.move_cursor_to(x, y+1).unwrap(); 
           print!("{}", style.apply_to("    "));                 
         }

//         x+=4;
      }
//      x=0;
//      y+=2;
    } 
//}
}


}
