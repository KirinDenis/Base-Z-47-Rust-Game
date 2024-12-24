mod backgroundthread;
mod drawlevel;
mod level1;
mod level2;
use console::Term;
use console::Style;
use console::Color;
use std::thread;
use std::time::Duration;
//use rodio::{OutputStream, source::SineWave};
//use rodio::Source;

struct Position {
      x: usize,
      y: usize
}

fn getHeroPos(level: [[char; 20]; 20]) -> Position {
  let mut pos = Position{x: 0, y : 0};

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
    return pos; 
}

fn canStep(level: [[char; 20]; 20], pos: Position) -> bool {

    if level[pos.x][pos.y] != ' ' {
       return false;
    }
    else  {
       return true;
    }
}


fn main() {
    
    let term = Term::stdout();
    term.clear_screen().unwrap();
    term.hide_cursor();

    backgroundthread::run();

    let mut level: [[char; 20]; 20] = level1::get();

    let mut heroPos = getHeroPos(level);       
    drawlevel::draw(level);
    print!("x - {} ", heroPos.x);
    print!("y - {} ", heroPos.y);


    
    loop {
            if let Ok(c) = term.read_char() {
               print!("{}", c);
               if c == 'q' {
                 break;
               }
               else 
               if c == '1' {
		  level = level1::get();    
                  heroPos = getHeroPos(level);       
                  drawlevel::draw(level);
               }
	       else  	
               if c == '2' {
		  level = level2::get();           
                  heroPos = getHeroPos(level);       
                  drawlevel::draw(level);
               }
               else 
               if c == 'w' {
                 if canStep(level, Position{x: heroPos.x -1, y: heroPos.y}) {
                 level[heroPos.x][heroPos.y] = ' ';
                 heroPos.x -= 1;
                 level[heroPos.x][heroPos.y] = '@';
                 drawlevel::draw(level);
                 }
               }
               else 
               if c == 's' {
                 if canStep(level, Position{x: heroPos.x +1, y: heroPos.y}) {
                 level[heroPos.x][heroPos.y] = ' ';
                 heroPos.x += 1;
                 level[heroPos.x][heroPos.y] = '@';
                 drawlevel::draw(level);
                 }
               }
               else 
               if c == 'a' {
                 if canStep(level, Position{x: heroPos.x, y: heroPos.y - 1}) {
                 level[heroPos.x][heroPos.y] = ' ';
                 heroPos.y -= 1;
                 level[heroPos.x][heroPos.y] = '@';
                 drawlevel::draw(level);
                 }
               }
               else 
               if c == 'd' {
                 if canStep(level, Position{x: heroPos.x, y: heroPos.y + 1}) {
                 level[heroPos.x][heroPos.y] = ' ';
                 heroPos.y += 1;
                 level[heroPos.x][heroPos.y] = '@';
                 drawlevel::draw(level);
                 }
               }




            }

     thread::sleep(Duration::from_millis(50));   
    }


    /*
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SineWave::new(400.0).take_duration(Duration::from_millis(70));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(300.0).take_duration(Duration::from_millis(50));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(550.0).take_duration(Duration::from_millis(70));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(450.0).take_duration(Duration::from_millis(50));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(400.0).take_duration(Duration::from_millis(70));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    let source = SineWave::new(300.0).take_duration(Duration::from_millis(50));
    stream_handle.play_raw(source.convert_samples()).unwrap();
    */

    /*
    for c in (0..255) {
    let style: Style = Style::new()
    .fg(Color::Color256(255-c))
    .bg(Color::Color256(c));

    print!("{}", style.apply_to("0"));
    }
    */

}
