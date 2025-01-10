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


struct Position {
      x: usize,
      y: usize
}


fn modify_level(level_name: &str, x: usize, y: usize, value: char) {
    let mut levels = LEVELS.lock().unwrap();
    if let Some(level) = levels.get_mut(level_name) {
        level[x][y] = value;
    } else {
        println!("Level {} not found!", level_name);
    }
}

fn getHeroPos(level_name: &str) -> Position {

    let mut pos = Position{x: 0, y : 0};
    let mut levels = LEVELS.lock().unwrap();
    if let Some(level) = levels.get_mut(level_name) {

    let mut x = 0;
    let mut y = 0;

    for row in level.iter(){
      for &cell in row.iter() {
         if cell == '@' //Wall
         {
            pos.x = x;
            pos.y = y;
            return pos;
         }
        y += 1;
       }
       y = 0;
       x += 1;
     } 
    }
    return pos; 
}

fn canStep(level_name: &str, x: isize, y: isize) -> bool {

    let mut pos = getHeroPos(level_name);       
    let mut levels = LEVELS.lock().unwrap();
    if let Some(level) = levels.get_mut(level_name) {

    //pos.x += x as usize;
    //pos.y += y as usize;

      println!();
      print!("x={}", pos.x);
      print!("y={}", pos.y);
      print!("position char ={}", level[pos.x][pos.y]);    

      println!();

      println!("position char ={}", level[pos.x][pos.y-1]);    
      println!("position char ={}", level[pos.x][pos.y-2]);    
      println!("position char ={}", level[pos.x][pos.y-3]);    
      println!();

      println!("position char ={}", level[pos.x][pos.y+1]);    
      println!("position char ={}", level[pos.x][pos.y+2]);    
      println!("position char ={}", level[pos.x][pos.y+3]);    
      println!();

    if y == -1 {
      pos.y -= 1;
      print!("x={}", pos.x);
      print!("y={}", pos.y);
    }

     print!("position char ={}", level[pos.x][pos.y]);    

    if level[pos.x][pos.y] == ' ' || level[pos.x][pos.y] == '.' {
       //level[pos.x][pos.y] = '@';
       return true;
    }
   }
   return false;
}



fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.hide_cursor();

    backgroundthread::run();

    
    let mut level = "level1";
  
    let mut heroPos = getHeroPos(level);       
    drawlevel::draw(level);

    loop {

            if let Ok(c) = term.read_char() {
               print!("{}", c);
               if c == 'q' {
                 break;
               }
               else 
               if c == '1' {

                  level = "level1";
                  heroPos = getHeroPos(level);       
                  drawlevel::draw(level);
               }
	       else  	
               if c == '2' {
                  level = "level2";
                  heroPos = getHeroPos(level);       
                  drawlevel::draw(level);
               }
               else 
               if c == 'w' {
                 if canStep(level, 0, -1) {
                 modify_level(level,heroPos.x, heroPos.y, ' ');
                 heroPos.x -= 1;
                 modify_level(level,heroPos.x, heroPos.y, '@');
                 drawlevel::draw(level);
                 }
               }
/*
               else 
               if c == 's' {
                 if canStep(level, &heroPos, 1, 0) {
                 //level[heroPos.x][heroPos.y] = ' ';
                 //heroPos.x += 1;
                 //level[heroPos.x][heroPos.y] = '@';
                 drawlevel::draw(level);
                 }
               }
               else 
               if c == 'a' {
                 if canStep(level, &heroPos, 0, -1) {
                 //level[heroPos.x][heroPos.y] = ' ';
                 //heroPos.y -= 1;
                 //level[heroPos.x][heroPos.y] = '@';
                 drawlevel::draw(level);
                 }
               }
               else 
               if c == 'd' {
                 if canStep(level, &heroPos, 0, 1) {
                 //level[heroPos.x][heroPos.y] = ' ';
                 //heroPos.y += 1;
                 //level[heroPos.x][heroPos.y] = '@';
                 drawlevel::draw(level);
                 }
               }



*/
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
