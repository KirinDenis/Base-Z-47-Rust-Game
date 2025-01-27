mod backgroundthread;
mod levels;
mod view;
mod model;
mod sound;

//use std::thread;
//use std::time::Duration;



use crate::model::do_step;


fn main() {

    view::init();

//    backgroundthread::run();

    levels::load_level("level1");
    view::draw();
    let mut levelindex = 1;

    loop {

            let c = view::read_char();
            //if let Ok(c) = term.read_char() 
             {
              // print!("{}", c);
               if c == 'q' {
                 break;
               }
               else 
               if c == '1' {
                  levelindex = levelindex - 1;
		  levels::load_level(&format!("level{}", levelindex));

                  view::draw();
                  sound::new_level_sound2();
               }
	       else  	
               if c == '2' {
                  levelindex = levelindex + 1;
		  levels::load_level(&format!("level{}", levelindex));
		  view::clear();
                  view::draw();
                  sound::new_level_sound();

               }
               else 
               if c == 'w' {
                 do_step(-1, 0);
               }
               else 
               if c == 's' {
                 do_step(1, 0);     

               }
               else 
               if c == 'a' {
                 do_step(0, -1);    

               }
               else 
               if c == 'd' {
                 do_step(0, 1);     

               }
              view::draw();          
            }
    }

}
