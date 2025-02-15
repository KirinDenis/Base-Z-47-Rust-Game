mod backgroundthread;
mod levels;
mod model;
mod sound;
mod view;

 //Background sound
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use crate::model::do_step;
use crossterm::event::KeyCode;


const UP_KEY: KeyCode = KeyCode::Up;
const DOWN_KEY: KeyCode = KeyCode::Down;
const LEFT_KEY: KeyCode = KeyCode::Left;
const RIGHT_KEY: KeyCode = KeyCode::Right;

const SELECT_KEY: KeyCode = KeyCode::Enter;
const QUIT_KEY: KeyCode = KeyCode::Esc;


fn select_level(levelindex: usize) {
    view::draw_image(0);
    let mut x_offset = 10;
    let y_offset = 25;
    for i in levelindex..levelindex + 3 {
        levels::load_level(&format!("level{}", i));
        view::custom_draw(x_offset, y_offset, i, true, i == levelindex);

        x_offset = x_offset + 60;
    }
    //    sound::new_level_sound2();
}



fn main() {

    if !view::init() {
        return;
    }
    view::intro_image(0); 


    //backgroundthread::run();
    let mut mode: usize = 0;
    let mut levelindex = 1;
    let mut step_result: usize = model::NO_STEP;
    select_level(levelindex);

   //Background sound play
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));


        let files = vec!["assets/music.mp3", "assets/music.mp3"];
        for file in &files {
            let file = BufReader::new(File::open(file).unwrap());
            let source = Decoder::new(file).unwrap().repeat_infinite();
            sink.lock().unwrap().append(source);
        }

        let sink = sink.lock().unwrap();
        sink.set_volume(0.1);

    

    loop {
        let key = view::read_char();

        if key == KeyCode::Char('_') {
            continue;
        }

        if key == KeyCode::Char('p') {
            if sink.is_paused() {
                            sink.play();
                        } else {
                            sink.pause();
                        }
            } else if key == KeyCode::Char('+') {
                let vol = sink.volume() + 0.1;
                sink.set_volume(vol.max(1.0));
            } else if key == KeyCode::Char('-') {
                let vol = sink.volume() - 0.1;
                sink.set_volume(vol.max(0.0));
            } 
 

        if mode == 0 {
            if key == LEFT_KEY {
                levelindex = levelindex - 1;
                select_level(levelindex);
            } else if key == RIGHT_KEY {
                levelindex = levelindex + 1;
                select_level(levelindex);
            }else if key == SELECT_KEY || key == KeyCode::Char(' ') {
                mode = 1;
                view::set_level(levelindex);
            }else if key == QUIT_KEY {            
            break;
            }
        } else {
            if key == KeyCode::Char('1') {
                levelindex = levelindex - 1;
                view::set_level(levelindex);
            } else if key == KeyCode::Char('2') {
                levelindex = levelindex + 1;
                view::set_level(levelindex);
            } else if key == UP_KEY {
                step_result = do_step(-1, 0);
            } else if key == DOWN_KEY {
                step_result = do_step(1, 0);
            } else if key == LEFT_KEY {
                step_result = do_step(0, -1);
            } else if key == RIGHT_KEY {
                step_result = do_step(0, 1);
            }if key == QUIT_KEY {            
                mode = 0;
               select_level(levelindex);
            }


            if step_result == model::NO_STEP {
                //  sound::bell_sound();
            } else if step_result == model::CAN_STEP {
                sound::step_sound();
                if !view::draw() {
                    break;
                }
            } else if step_result == model::NEXT_LEVEL {
                levelindex = levelindex + 1;
                view::set_level(levelindex);
            }

            step_result = 0;
        }
    }
}
