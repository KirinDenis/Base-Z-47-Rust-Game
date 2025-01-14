mod backgroundthread;
mod levels;
mod drawlevel;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use console::Term;
use console::Style;
use console::Color;
use std::thread;
use std::time::Duration;


use crate::levels::LEVELS;
use crate::levels::CLEVELS;


struct Position {
      row: usize,
      col: usize
}


fn modify_level(level_name: &str, row: usize, col: usize, value: char) {
    let mut levels = LEVELS.lock().unwrap();
    if let Some(level) = levels.get_mut(level_name) {
        level[row][col] = value;
    } else {
     //   println!("Level {} not found!", level_name);
    }
}

fn get_hero_pos(level_name: &str) -> Position {

    let mut levels = LEVELS.lock().unwrap();
    if let Some(level) = levels.get_mut(level_name) {

    let mut _row = 0;
    let mut _col = 0;

    for row in level.iter(){
      for &cell in row.iter() {
         if cell == '@' //Wall
         {
            return Position{row: _row, col: _col};
         }
        _col += 1;
       }
       _col = 0;
       _row += 1;
     } 
    }
    return Position{row: 0, col: 0}; 
}

fn update_pos(row: isize, col: isize, mut pos: Position) -> Position {


       if row > 0 {
         pos.row += 1;
       }
       else 
       if row < 0 {
         pos.row -= 1;
       }

       if col > 0 {
         pos.col += 1;
       }
       else 
       if col < 0 {
         pos.col -= 1;
       }
    return pos; 
}

fn can_step(level_name: &str, row: isize, col: isize) -> bool {

    let mut pos = get_hero_pos(level_name);       

    let mut levels = LEVELS.lock().unwrap();
    if let Some(level) = levels.get_mut(level_name) {

    pos = update_pos(row, col, pos);  

    if level[pos.row][pos.col] == ' ' || level[pos.row][pos.col] == '.' {
       return true;
    }
   }
   return false;
}

fn do_step(level_name: &str, row: isize, col: isize) -> bool {
     if can_step(level_name,  row, col) {
       let mut hero_pos = get_hero_pos(level_name);       
       modify_level(level_name,hero_pos.row, hero_pos.col, ' ');

       hero_pos  = update_pos(row, col, hero_pos);  

       modify_level(level_name,hero_pos.row, hero_pos.col, '@');
       drawlevel::draw(level_name);
     }
    return true;
}


fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.hide_cursor();

    backgroundthread::run();

    
    let mut level_name = "level1";
  
    let mut hero_pos = get_hero_pos(level_name);       
    drawlevel::draw(level_name);

    loop {

            if let Ok(c) = term.read_char() {
               print!("{}", c);
               if c == 'q' {
                 break;
               }
               else 
               if c == '1' {

                  level_name = "level1";
                  hero_pos = get_hero_pos(level_name);       
                  drawlevel::draw(level_name);
               }
	       else  	
               if c == '2' {

	    let mut levels = LEVELS.lock().unwrap();
	    if let Some(level) = levels.get_mut("level2") {
              let mut cls = CLEVELS.lock().unwrap();
              if let mut cl = cls {
                  //  cl =  level;
                 // cl[1][1] = '#';

       for y in (0..19) {
        for x in (0..19) {
          cl[y][x] = level[y][x];
	}
	}

		}
	      }           

                  //level_name = "level2";
                  //hero_pos = get_hero_pos(level_name);       
                  drawlevel::draw(level_name);
               }
               else 
               if c == 'w' {
                 do_step(level_name,  -1, 0);
               }
               else 
               if c == 's' {
                 do_step(level_name,  1, 0);               
               }
               else 
               if c == 'a' {
                 do_step(level_name,  0, -1);                                
               }
               else 
               if c == 'd' {
                 do_step(level_name,  0, 1);                                                 
               }
            }

     thread::sleep(Duration::from_millis(50));   
    }




/*
    println!("Level 1:");
    drawlevel::draw("level1");


    modify_level("level1", 2, 2, 'X');
    println!("\nModified Level 1:");
    drawlevel::draw("level1");


    println!("\nLevel 2:");
    drawlevel::draw("level2");
*/
}
