
use console::Term;
use console::Style;
use console::Color;
use std::thread;
use std::time::Duration;

use crate::images::draw;

pub fn run() {

    thread::spawn(|| {

       let mut c_color = 1;
       let term = Term::stdout();
       let mut flag = true;
 
       loop {


           term.move_cursor_to(0, 0).unwrap(); 
          draw(2);

          thread::sleep(Duration::from_millis(1000));          
        }

    });

}