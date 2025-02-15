/*
use console::Term;
use std::thread;
use std::time::Duration;



pub fn run() {

    thread::spawn(|| {
       
       let term = Term::stdout();
       
 
       loop {


           term.move_cursor_to(0, 0).unwrap(); 
//          draw(2);

          thread::sleep(Duration::from_millis(1000));          
        }

    });

}
     */