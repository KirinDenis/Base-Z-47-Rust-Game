mod image1;
mod image2;
mod image3;

use ansi_colours::*;
use console::Color;
use console::Style;
use console::Term;
use rgb::RGB8;


pub fn draw(image: usize) {
    let term = Term::stdout();
     if term.is_term() {
        //let (width, height) = term.size();
    } else {
        eprintln!("not term");
    }

    let (_sh, _sw) = term.size();
    let style: Style = Style::new();

    let pixels;
    if image == 1 { 
      pixels = image1::get();
    }
    else 
    if image == 2 { 
      pixels = image2::get();
    }
    else 
    {
      pixels = image3::get();
    }



    term.move_cursor_to(0, 0).unwrap();    

    let mut psize = 60000;
    let mut lsize = 200 * 3;

     
   for i in (0..psize).step_by(lsize*2) {    
     for j in (i..i + lsize).step_by(3) {
    	let fc: RGB8 = RGB8 {
	    r: pixels[j+0],
	    g: pixels[j+1]  ,
	    b: pixels[j+2] << 1,
	    };

    	let bc: RGB8 = RGB8 {
	    r: pixels[j+0 + lsize],
	    g: pixels[j+1 + lsize]  ,
	    b: pixels[j+2 + lsize] << 1,
	    };
    let style: Style = Style::new()
        .fg(Color::Color256(ansi256_from_rgb(fc)))
        .bg(Color::Color256(ansi256_from_rgb(bc)));


    print!("{}", style.apply_to("\u{2580}"));


     }


   }
    
 	

/*
    for i in (0..25310).step_by(3) {
    let pc: RGB8 = RGB8 {
    r: pixels[i+0],
    g: pixels[i+1],
    b: pixels[i+2],
    };
    let style: Style = Style::new()
        .fg(Color::Color256(ansi256_from_rgb(pc)))
        .bg(Color::Color256(ansi256_from_rgb(pc)));


    print!("{}", style.apply_to(" "));
       


    }
*/

/*
    for i in 0..=255 {

    let style: Style = Style::new()
        .bg(Color::Color256(i));
    
     print!("{}", style.apply_to(" "));
    }
    println!("{}", _sw);
    println!(" ");
    println!("{}", _sh);
*/
   
}
