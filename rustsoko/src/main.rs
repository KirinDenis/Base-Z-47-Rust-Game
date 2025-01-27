mod backgroundthread;
mod levels;
mod view;
mod model;
mod sound;

//use std::thread;
//use std::time::Duration;

use crate::model::do_step;


fn set_level(levelindex: usize) {

                  
		  levels::load_level(&format!("level{}", levelindex));
		  view::clear();
                  view::draw();
                  sound::new_level_sound2();

}


fn main() {

    view::init();

//    backgroundthread::run();

    let mut levelindex = 1;
    let mut step_result = 0;
    set_level(levelindex);
    loop {

            let c = view::read_char();

             {
              // print!("{}", c);
               if c == 'q' {
                 break;
               }
               else 
               if c == '1' {
                  levelindex = levelindex - 1;
                  set_level(levelindex);
               }
	       else  	
               if c == '2' {
                  levelindex = levelindex + 1;
                  set_level(levelindex);
               }
               else 
               if c == 'w' {
                // step_result = do_step(-1, 0);
                   step_result = do_step(-1, 0);
               }
               else 
               if c == 's' {
                 step_result = do_step(1, 0);     

               }
               else 
               if c == 'a' {
                 step_result = do_step(0, -1);    

               }
               else 
               if c == 'd' {
                 step_result = do_step(0, 1);     

               }

	  if  step_result == 0 {
	      sound::bell_sound(); 
	  }
	  else 
	   if step_result == 1 {
	      sound::step_sound(); 
	      view::draw();
	   } 
	   else 
	   {
                 levelindex = levelindex + 1;
                 set_level(levelindex);    
	   }

           step_result = 0; 
            }
    }

}
