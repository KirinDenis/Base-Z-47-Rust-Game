use console::Term;
use console::Style;
use console::Color;
use std::thread;
use std::time::Duration;

pub fn run() {

    thread::spawn(|| {

       let mut c_color = 1;
       let term = Term::stdout();
       let mut flag = true;
 
       loop {


        term.move_cursor_to(0, 0).unwrap(); 
    	for c in (0..36) {


	   // .bg(Color::Color256(c));
            let style: Style = Style::new().fg(Color::Color256(c+(c_color*36)+16));


	    print!("{}", style.apply_to("0"));

            //thread::sleep(Duration::from_millis(20));          
          }

          if flag 
          {
           c_color +=1;
           if c_color > 4 {
              flag = !flag;
           }
          }
          else 
          {
            c_color -=1;
            if c_color < 1 {
               flag = !flag;             
            }             
          }
	  //  println!("{}", c_color);

          thread::sleep(Duration::from_millis(500));          
        }

    });

}