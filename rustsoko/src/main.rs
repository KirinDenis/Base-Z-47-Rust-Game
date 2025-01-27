mod backgroundthread;
mod levels;
mod view;
mod model;
mod sound;

//use once_cell::sync::Lazy;
//use std::collections::HashMap;
//use std::sync::Mutex;
use console::Term;
//use console::Style;
//use console::Color;
use std::thread;
use std::time::Duration;


//use crate::levels::load_level;
use crate::model::do_step;
//use crate::sound::new_level_sound;
//use crate::sound::new_level_sound2;


fn main() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.hide_cursor();

//    backgroundthread::run();

    
   // let mut level_name = "level1";
  
//    let mut hero_pos = get_hero_pos(level_name);       
    levels::load_level("level1");
    view::draw();
    let mut levelindex = 1;

    loop {

            //   print!("{}", levelindex);
            if let Ok(c) = term.read_char() {
              // print!("{}", c);
               if c == 'q' {
                 break;
               }
               else 
               if c == '1' {
                  levelindex = levelindex - 1;
		  levels::load_level(&format!("level{}", levelindex));
//                  level_name = "level1";
//                  hero_pos = get_hero_pos(level_name);       
                  term.clear_screen().unwrap();
                  view::draw();
                  sound::new_level_sound2();
               }
	       else  	
               if c == '2' {
                  levelindex = levelindex + 1;
		  levels::load_level(&format!("level{}", levelindex));
                  //level_name = "level2";
                  //hero_pos = get_hero_pos(level_name);       
                  term.clear_screen().unwrap();
                  view::draw();
                  sound::new_level_sound();

               }
               else 
               if c == 'w' {
                 do_step(-1, 0);
//       drawlevel::draw();
               }
               else 
               if c == 's' {
                 do_step(1, 0);     
//                  drawlevel::draw();          
               }
               else 
               if c == 'a' {
                 do_step(0, -1);    
//                  drawlevel::draw();                            
               }
               else 
               if c == 'd' {
                 do_step(0, 1);     
//                  drawlevel::draw();                                            
               }
              view::draw();          
            }



//     thread::sleep(Duration::from_millis(50));   
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
