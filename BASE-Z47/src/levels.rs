mod level1;
mod level10;
mod level11;
mod level12;
mod level13;
mod level14;
mod level15;
mod level16;
mod level17;
mod level18;
mod level19;
mod level2;
mod level20;
mod level21;
mod level22;
mod level23;
mod level24;
mod level25;
mod level26;
mod level27;
mod level28;
mod level29;
mod level3;
mod level30;
mod level31;
mod level32;
mod level33;
mod level34;
mod level35;
mod level36;
mod level37;
mod level38;
mod level39;
mod level4;
mod level40;
mod level41;
mod level42;
mod level43;
mod level44;
mod level45;
mod level46;
mod level47;
mod level48;
mod level49;
mod level5;
mod level50;
mod level51;
mod level52;
mod level53;
mod level54;
mod level55;
mod level56;
mod level57;
mod level58;
mod level59;
mod level6;
mod level60;
mod level7;
mod level8;
mod level9;
pub mod level_const;

use level_const::BOX_CODE;
use level_const::FLOOR_CODE;
use level_const::HERO_CODE;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use level_const::L_WIDTH;
use level_const::L_HEIGHT;

pub const CURRENT_LEVEL_NAME: &str = "current_level";
pub const ORIGINAL_LEVEL_NAME: &str = "original_level";

pub type Level = [[char; L_WIDTH]; L_HEIGHT];

pub static CLEVEL: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut clevel = HashMap::new();

    clevel.insert(CURRENT_LEVEL_NAME.to_string(), level1::get());
    Mutex::new(clevel)
});

pub static OLEVEL: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut olevel = HashMap::new();

    olevel.insert(ORIGINAL_LEVEL_NAME.to_string(), level1::get());
    Mutex::new(olevel)
});

static LEVELS: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut levels = HashMap::new();

    levels.insert("level1".to_string(), level1::get());
    levels.insert("level2".to_string(), level2::get());
    levels.insert("level3".to_string(), level3::get());
    levels.insert("level4".to_string(), level4::get());
    levels.insert("level5".to_string(), level5::get());
    levels.insert("level6".to_string(), level6::get());
    levels.insert("level7".to_string(), level7::get());
    levels.insert("level8".to_string(), level8::get());
    levels.insert("level9".to_string(), level9::get());
    levels.insert("level10".to_string(), level10::get());
    levels.insert("level11".to_string(), level11::get());
    levels.insert("level12".to_string(), level12::get());
    levels.insert("level13".to_string(), level13::get());
    levels.insert("level14".to_string(), level14::get());
    levels.insert("level15".to_string(), level15::get());
    levels.insert("level16".to_string(), level16::get());
    levels.insert("level17".to_string(), level17::get());
    levels.insert("level18".to_string(), level18::get());
    levels.insert("level19".to_string(), level19::get());
    levels.insert("level20".to_string(), level20::get());
    levels.insert("level21".to_string(), level21::get());
    levels.insert("level22".to_string(), level22::get());
    levels.insert("level23".to_string(), level23::get());
    levels.insert("level24".to_string(), level24::get());
    levels.insert("level25".to_string(), level25::get());
    levels.insert("level26".to_string(), level26::get());
    levels.insert("level27".to_string(), level27::get());
    levels.insert("level28".to_string(), level28::get());
    levels.insert("level29".to_string(), level29::get());
    levels.insert("level30".to_string(), level30::get());
    levels.insert("level31".to_string(), level31::get());
    levels.insert("level32".to_string(), level32::get());
    levels.insert("level33".to_string(), level33::get());
    levels.insert("level34".to_string(), level34::get());
    levels.insert("level35".to_string(), level35::get());
    levels.insert("level36".to_string(), level36::get());
    levels.insert("level37".to_string(), level37::get());
    levels.insert("level38".to_string(), level38::get());
    levels.insert("level39".to_string(), level39::get());
    levels.insert("level40".to_string(), level40::get());
    levels.insert("level41".to_string(), level41::get());
    levels.insert("level42".to_string(), level42::get());
    levels.insert("level43".to_string(), level43::get());
    levels.insert("level44".to_string(), level44::get());
    levels.insert("level45".to_string(), level45::get());
    levels.insert("level46".to_string(), level46::get());
    levels.insert("level47".to_string(), level47::get());
    levels.insert("level48".to_string(), level48::get());
    levels.insert("level49".to_string(), level49::get());
    levels.insert("level50".to_string(), level50::get());
    levels.insert("level51".to_string(), level51::get());
    levels.insert("level52".to_string(), level52::get());
    levels.insert("level53".to_string(), level53::get());
    levels.insert("level54".to_string(), level54::get());
    levels.insert("level55".to_string(), level55::get());
    levels.insert("level56".to_string(), level56::get());
    levels.insert("level57".to_string(), level57::get());
    levels.insert("level58".to_string(), level58::get());
    levels.insert("level59".to_string(), level59::get());
    levels.insert("level60".to_string(), level60::get());

    Mutex::new(levels)
});

pub fn load_level(level_name: &str) {
    let mut levels = LEVELS.lock().unwrap();
    let mut clevel = CLEVEL.lock().unwrap();
    let mut olevel = OLEVEL.lock().unwrap();
    if let Some(level) = levels.get_mut(level_name) {
        if let Some(clevel) = clevel.get_mut(CURRENT_LEVEL_NAME) {
            if let Some(olevel) = olevel.get_mut(ORIGINAL_LEVEL_NAME) {
                for y in 0..L_HEIGHT {
                    for x in 0..L_WIDTH {
                        clevel[y][x] = level[y][x];

                        if level[y][x] != HERO_CODE && level[y][x] != BOX_CODE {
                            olevel[y][x] = level[y][x];
                        } else {
                            olevel[y][x] = FLOOR_CODE;
                        }
                    }
                }
            }
        }
    }
}

//pub fn get_current_level() -> Level {
//let mut clevel = CLEVEL.lock().unwrap();
//	     	let mut current_level = **&CLEVEL;

//                return CLEVEL;// *CLEVEL.lock().unwrap();
//}
