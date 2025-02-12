mod backgroundthread;
mod images;
mod levels;
mod model;
mod sound;
mod view;

/* //Background sound
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
 */

use levels::level_const::S_WIDTH;

use crate::model::do_step;

const UP_KEY: char = 'w';
const DOWN_KEY: char = 's';
const LEFT_KEY: char = 'a';
const RIGHT_KEY: char = 'd';

const QUIT_KEY: char = 'q';

fn set_level(levelindex: usize) {
    levels::load_level(&format!("level{}", levelindex));
    view::clear();
    images::draw(levelindex);

    view::draw();

    //    sound::new_level_sound2();
}

fn select_level(levelindex: usize) {
    images::draw(0);
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

    //backgroundthread::run();
    let mut mode: usize = 0;
    let mut levelindex = 1;
    let mut step_result: usize = model::NO_STEP;
    select_level(levelindex);

    /*  //Background sound play
        let (_stream, handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));


        let files = vec!["assets/music.mp3", "assets/music.mp3"];
        for file in &files {
            let file = BufReader::new(File::open(file).unwrap());
            let source = Decoder::new(file).unwrap().repeat_infinite();
            sink.lock().unwrap().append(source);
        }

        let mut sink = sink.lock().unwrap();
        sink.set_volume(0.4);

    /*if key == 'p' {
            if sink.is_paused() {
                            sink.play();
                        } else {
                            sink.pause();
                        }
            } else if key == '+' {
                let vol = sink.volume() + 0.1;
                sink.set_volume(vol.max(1.0));
            } else if key == '-' {
                let vol = sink.volume() - 0.1;
                sink.set_volume(vol.max(0.0));
            } else */
    */

    loop {
        let key = view::read_char();

        if key == QUIT_KEY {
            break;
        } else if key == ' ' {
            continue;
        }

        if mode == 0 {
            if key == LEFT_KEY {
                levelindex = levelindex - 1;
                select_level(levelindex);
            } else if key == RIGHT_KEY {
                levelindex = levelindex + 1;
                select_level(levelindex);
            }else if key == UP_KEY {
                mode = 1;
                set_level(levelindex);
            }
        } else {
            if key == '1' {
                levelindex = levelindex - 1;
                set_level(levelindex);
            } else if key == '2' {
                levelindex = levelindex + 1;
                set_level(levelindex);
            } else if key == UP_KEY {
                step_result = do_step(-1, 0);
            } else if key == DOWN_KEY {
                step_result = do_step(1, 0);
            } else if key == LEFT_KEY {
                step_result = do_step(0, -1);
            } else if key == RIGHT_KEY {
                step_result = do_step(0, 1);
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
                set_level(levelindex);
            }

            step_result = 0;
        }
    }
}
