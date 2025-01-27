//mod levels;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;


//use crate::levels::LEVELS;
use crate::levels::CLEVEL;
use crate::levels::OLEVEL;
//use crate::levels::get_current_level;


struct Position {
      row: usize,
      col: usize
}


fn modify_level(row: usize, col: usize, value: char) {

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level") {

        level[row][col] = value;
       }       
}

fn get_hero_pos() -> Position {

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level") {
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
    //}
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

fn can_step(row: isize, col: isize) -> bool {

    let mut pos = get_hero_pos();       

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level") {


    pos = update_pos(row, col, pos);  

    if level[pos.row][pos.col] == ' ' || level[pos.row][pos.col] == '.' {
       return true;
    }
    else 
    if level[pos.row][pos.col] == '$' {
      pos = update_pos(row, col, pos);  
      if level[pos.row][pos.col] == ' ' || level[pos.row][pos.col] == '.' {
        level[pos.row][pos.col] = '$';
        return true;
      }
    }
    
   }
   return false;
}

pub fn do_step(row: isize, col: isize) -> usize {
	let mut step_result = 0; 
	if can_step(row, col) {
     
		let mut hero_pos = get_hero_pos();       

		let mut clevel = CLEVEL.lock().unwrap();
		if let Some(clevel) = clevel.get_mut("current_level") {
			let mut olevel = OLEVEL.lock().unwrap();
			if let Some(olevel) = olevel.get_mut("original_level") {

				if olevel[hero_pos.row][hero_pos.col] != '$' {
					clevel[hero_pos.row][hero_pos.col] = olevel[hero_pos.row][hero_pos.col];
				 }
			}

			 hero_pos  = update_pos(row, col, hero_pos);  

			clevel[hero_pos.row][hero_pos.col] = '@';

			step_result = 1;
		 }
	}

	if check_win() {
		step_result = 2;
	}  

       step_result
}

fn check_win() -> bool {
    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(clevel) = clevel.get_mut("current_level") {
    let mut olevel = OLEVEL.lock().unwrap();
    if let Some(olevel) = olevel.get_mut("original_level") {
       for y in (0..20) {
        for x in (0..30) {
           if clevel[y][x] == '$' && olevel[y][x] != '.' {
             return false;
           }
        } 
       }
     }
    }
   return true;
}

