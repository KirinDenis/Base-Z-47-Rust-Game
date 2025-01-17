mod level1;
mod level2;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub type Level = [[char; 20]; 20];


pub static CLEVEL: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut clevel = HashMap::new();

    clevel.insert("current_level".to_string(), level1::get());
    Mutex::new(clevel)
});

pub static OLEVEL: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut olevel = HashMap::new();

    olevel.insert("original_level".to_string(), level1::get());
    Mutex::new(olevel)
});



static LEVELS: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut levels = HashMap::new();

    levels.insert("level1".to_string(), level1::get());
    levels.insert("level2".to_string(), level2::get());
    Mutex::new(levels)
});

pub fn load_level(level_name: &str) {

	let mut levels = LEVELS.lock().unwrap();
	let mut clevel = CLEVEL.lock().unwrap();
	let mut olevel = OLEVEL.lock().unwrap();
	if let Some(level) = levels.get_mut(level_name) {
	if let Some(clevel) = clevel.get_mut("current_level") {
	if let Some(olevel) = olevel.get_mut("original_level") {
       			for y in (0..20) {
        		for x in (0..20) {
          	   		clevel[y][x] = level[y][x];

                                if level[y][x] != '@' &&  level[y][x] != '$' {
				olevel[y][x] = level[y][x];
                                }
                                else { 
  				olevel[y][x] = ' '; 
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