
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use console::Term;
use console::Style;
use console::Color;
use rgb::RGB8;
use ansi_colours::*;

//use crate::levels::get_current_level;

use crate::levels::CLEVEL;

pub const F_LEVEL_COLOR : RGB8 = RGB8 { r: 0xF7, g: 0xF0, b: 0xD4 };
pub const B_LEVEL_COLOR : RGB8 = RGB8 { r: 0xF7, g: 0xF0, b: 0xD4};

pub const F_HERO_COLOR : RGB8 = RGB8 { r: 0xF7, g: 0xF0, b: 0xD4};
pub const B_HERO_COLOR : RGB8 = RGB8 { r: 0xC2, g: 0x14, b: 0x60};

pub const F_BLOCK_COLOR : RGB8 = RGB8 { r: 0x34, g: 0x7B, b: 0x98 };
pub const B_BLOCK_COLOR : RGB8 = RGB8 { r: 0xFC, g: 0xCB, b: 0x1A };

pub const F_BASE_COLOR : RGB8 = RGB8 { r: 0xC2, g: 0x14, b: 0x60};
pub const B_BASE_COLOR : RGB8 = RGB8 {  r: 0xF7, g: 0xF0, b: 0xD4};

pub const F_WALL_COLOR : RGB8 = RGB8 { r: 0x34, g: 0x7B , b: 0x98};
pub const B_WALL_COLOR : RGB8 = RGB8 { r: 0x34, g: 0x7B , b: 0x98};



fn get_style(foreground : RGB8, background: RGB8) -> Style {

     Style::new()
    .fg(Color::Color256(ansi256_from_rgb(foreground)))
    .bg(Color::Color256(ansi256_from_rgb(background)))
}


pub fn draw() {

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level") {


    let term = Term::stdout();
    let style: Style = Style::new();

//    let mut x = 0;
//    let mut y = 0;

//    for row in level.iter(){
//      for &cell in row.iter() {
       for y in (0..20) {
        for x in (0..30) {

         let cell = level[y][x];
         let sx = x * 5;
         let sy = y * 2;

          term.move_cursor_to(sx, sy).unwrap(); 	

         if cell == '#' //Wall
         {
           let style = get_style(F_WALL_COLOR, B_WALL_COLOR);
           print!("{}", style.apply_to("\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}"));           
           term.move_cursor_to(sx, sy+1).unwrap(); 
           print!("{}", style.apply_to("\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}"));           
         }
 	else 
         if cell == '.' //Base
         {
           let style = get_style(F_BASE_COLOR, B_BASE_COLOR);
           print!("{}", style.apply_to(" \u{250C}\u{2500}\u{2510} "));           
           term.move_cursor_to(sx, sy+1).unwrap(); 
           print!("{}", style.apply_to(" \u{2514}\u{2500}\u{2518} "));           
         }
 	else 
         if cell == '$' //Box
         {


           let style = get_style(F_BLOCK_COLOR, B_BLOCK_COLOR);

           print!("{}", style.apply_to(" \u{2554}\u{2550}\u{2557} "));           
           term.move_cursor_to(sx, sy+1).unwrap(); 
           print!("{}", style.apply_to(" \u{255A}\u{2550}\u{255D} "));           
         }         
 	else 
         if cell == '@' //Hero
         {

           let style = get_style(F_HERO_COLOR, B_HERO_COLOR);

           print!("{}", style.apply_to(" @@@ "));           
           term.move_cursor_to(sx, sy+1).unwrap(); 
           print!("{}", style.apply_to(" @@@ "));           
         }         
         else  //Default space 
	 {
           let style = get_style(F_LEVEL_COLOR, B_LEVEL_COLOR);
           print!("{}", style.apply_to("     "));           
           term.move_cursor_to(sx, sy+1).unwrap(); 
           print!("{}", style.apply_to("     "));                 
         }

//         x+=4;
      }
//      x=0;
//      y+=2;
    } 
//}
}


}
