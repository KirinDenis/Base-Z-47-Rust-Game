mod backgroundthread;
mod levels;
mod model;
mod sound;
mod view;
mod images;

//use std::thread;
//use std::time::Duration;

use crate::model::do_step;

const UP_KEY: char = 'w';
const DOWN_KEY: char = 's';
const LEFT_KEY: char = 'a';
const RIGHT_KEY: char = 'd';

const QUIT_KEY: char = 'q';


fn set_level(levelindex: usize) {
    levels::load_level(&format!("level{}", levelindex));
//    view::clear();
    images::draw(levelindex); 
    view::draw();
    sound::new_level_sound2();
}

fn main() {
    view::init();

    //    backgroundthread::run();

    let mut levelindex = 1;
    let mut step_result: usize= model::NO_STEP;
    set_level(levelindex);


    loop {
        let key = view::read_char();

        {            
            if key == QUIT_KEY {
                break;
            } else if key == '1' {
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
                sound::bell_sound();
            } else if step_result == model::CAN_STEP {
                sound::step_sound();
                view::draw();
            } else if step_result == model::NEXT_LEVEL {
                levelindex = levelindex + 1;
                set_level(levelindex);
            }

            step_result = 0;
        }
    }
}
