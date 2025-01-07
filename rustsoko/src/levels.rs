mod level1;
mod level2;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;


pub type Level = [[char; 20]; 20];


pub static LEVELS: Lazy<Mutex<HashMap<String, Level>>> = Lazy::new(|| {
    let mut levels = HashMap::new();

    levels.insert("level1".to_string(), level1::get());
    levels.insert("level2".to_string(), level2::get());
    Mutex::new(levels)
});
