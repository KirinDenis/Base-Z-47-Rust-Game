pub mod image0;
pub mod image1;
pub mod image2;
pub mod image3;
pub mod image4;

use ansi_colours::*;
use console::Color;
use console::Style;
use console::Term;
use rgb::RGB8;
use std::fs;


pub fn draw(image: usize) {
    let term = Term::stdout();
     if term.is_term() {
        //let (width, height) = term.size();
    } else {
        eprintln!("not term");
        return;
    }

    let mut buffer: Vec<String> = Vec::new();

    let (_sh, _sw) = term.size();
    
    let pixels;

    if image == 0 { 
      pixels = image0::get();
    }
    else 
    if image == 1 { 
      pixels = image0::get();
    }
    else 
    if image == 2 { 
      pixels = image2::get();
    }
    else 
    if image == 3 { 
      pixels = image3::get();
    }
    else 
    {
      pixels = image4::get();
    }

    //let rng = rand::thread_rng();

    term.move_cursor_to(0, 0).unwrap();    

    let psize = 56000;
    let lsize = 200 * 3;

    let contents = fs::read_to_string("assets/image1.txt")
        .expect("Should have been able to read the file");

    let chars: Vec<char> = contents.chars().collect();
    let mut count: usize = 0;
     
   for i in (0..psize).step_by(lsize*2) {    
     let mut s1: String = Default::default();          

     for j in (i..i + lsize).step_by(3) {


//  let r = rng.gen_range(0..=3);
//if r < 2 {
    	let fc: RGB8 = RGB8 {
	    r: pixels[j+0] ,
	    g: pixels[j+1]    ,
	    b: pixels[j+2],
	    };

    	let bc: RGB8 = RGB8 {
	    r: pixels[j+0 + lsize] ,
	    g: pixels[j+1 + lsize]  ,
	    b: pixels[j+2 + lsize] ,
	    };

    
    let style: Style = Style::new()
        .fg(Color::Color256(ansi256_from_rgb(fc)))
        .bg(Color::Color256(ansi256_from_rgb(bc)));

        //print!("{}", chars[count]);
       // print!("{}", style.apply_to(rng.gen_range('A'..='F')));
      s1.push_str(&style.apply_to("\u{2580}").to_string());
//      s1.push_str(&style.apply_to({bc}).to_string());
      //^^^Print colors
      //s1.push_str(&style.apply_to(chars[count]).to_string());

//    }
/*
    else {
    	let fc: RGB8 = RGB8 {
	    r: pixels[j+0] << 1,
	    g: pixels[j+1] << 1 ,
	    b: pixels[j+2] << 1,
	    };

    	let bc: RGB8 = RGB8 {
	    r: pixels[j+0 + lsize],
	    g: pixels[j+1 + lsize]  ,
	    b: pixels[j+2 + lsize] ,
	    };

    
    let style: Style = Style::new()
        .fg(Color::Color256(ansi256_from_rgb(fc)))
        .bg(Color::Color256(ansi256_from_rgb(bc)));
 //       print!("{}", style.apply_to("\u{2580}"));
//        print!("{}", style.apply_to(rng.gen_range('0'..='9')));
//         print!("{}", style.apply_to(' '));
      //s1.push_str(&style.apply_to(' ').to_string());
        s1.push_str(&style.apply_to(chars[count]).to_string());

    }
*/
      count=count+1;
     }
     count=count+2;
     buffer.push(s1);

   }

        term.move_cursor_to(0, 0).unwrap();
        for text in buffer {
            term.write_line(&text).unwrap();
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
