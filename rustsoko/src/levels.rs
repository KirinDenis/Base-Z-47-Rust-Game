mod level1;
mod level2;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;


pub type Level = [[char; 20]; 20];


pub static CLEVELS: Lazy<Mutex<Level>> = Lazy::new(|| {

    let mut clevels = level1::get();

//    clevels.insert(1, level1::get());

    Mutex::new(clevels)

});


pub static LEVELS: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut levels = HashMap::new();

    levels.insert("level1".to_string(), level1::get());
    levels.insert("level2".to_string(), level2::get());
    Mutex::new(levels)
});
