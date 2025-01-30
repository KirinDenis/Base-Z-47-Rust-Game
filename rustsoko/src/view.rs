use ansi_colours::*;
use console::Color;
use console::Style;
use console::Term;
use rgb::RGB8;
use std::thread;
use crossterm::{ExecutableCommand, terminal};
use std::io::{self, Write};

use crate::levels::Level;
use crate::levels::CLEVEL;
use crate::images::image1;

pub const F_LEVEL_COLOR: RGB8 = RGB8 {
    r: 0xF7,
    g: 0xF0,
    b: 0xD4,
};
pub const B_LEVEL_COLOR: RGB8 = RGB8 {
    r: 0x00,
    g: 0x00,
    b: 0x00,
};

pub const F_FLOOR_COLOR: RGB8 = RGB8 {
    r: 0xF7,
    g: 0xF0,
    b: 0xD4,
};
pub const B_FLOOR_COLOR: RGB8 = RGB8 {
    r: 0xE7,
    g: 0xE0,
    b: 0xC4,
};

pub const F_SFLOOR_COLOR: RGB8 = RGB8 {
    r: 0xF7,
    g: 0xF0,
    b: 0xD4,
};
pub const B_SFLOOR_COLOR: RGB8 = RGB8 {
    r: 0xF7,
    g: 0xF0,
    b: 0xD4,
};

pub const F_HERO_COLOR: RGB8 = RGB8 {
    r: 0xF7,
    g: 0xF0,
    b: 0xD4,
};
pub const B_HERO_COLOR: RGB8 = RGB8 {
    r: 0xC2,
    g: 0x14,
    b: 0x60,
};

pub const F_BLOCK_COLOR: RGB8 = RGB8 {
    r: 0x34,
    g: 0x7B,
    b: 0x98,
};
pub const B_BLOCK_COLOR: RGB8 = RGB8 {
    r: 0xFC,
    g: 0xCB,
    b: 0x1A,
};

pub const F_SBLOCK_COLOR: RGB8 = RGB8 {
    r: 0x34,
    g: 0x7B,
    b: 0x98,
};
pub const B_SBLOCK_COLOR: RGB8 = RGB8 {
    r: 0xEC,
    g: 0xBB,
    b: 0x0A,
};

pub const F_BASE_COLOR: RGB8 = RGB8 {
    r: 0xC2,
    g: 0x14,
    b: 0x60,
};
pub const B_BASE_COLOR: RGB8 = RGB8 {
    r: 0xF7,
    g: 0xF0,
    b: 0xD4,
};

pub const F_WALL_COLOR: RGB8 = RGB8 {
    r: 0x34,
    g: 0x7B,
    b: 0x98,
};
pub const B_WALL_COLOR: RGB8 = RGB8 {
    r: 0x34,
    g: 0x7B,
    b: 0x98,
};

use crate::levels::level_const::L_HEIGHT;
use crate::levels::level_const::L_WIDTH;
use crate::levels::level_const::S_WIDTH;
use crate::levels::level_const::S_HEIGHT;

use crate::levels::level_const::BASE_CODE;
use crate::levels::level_const::BOX_CODE;
use crate::levels::level_const::FLOOR_CODE;
use crate::levels::level_const::HERO_CODE;


fn get_style(foreground: RGB8, background: RGB8) -> Style {
    Style::new()
        .fg(Color::Color256(ansi256_from_rgb(foreground)))
        .bg(Color::Color256(ansi256_from_rgb(background)))
}

pub fn init() -> bool {
    let term = Term::stdout();

    if !term.is_term()  {
        eprintln!("Terminal detection problem :: please run the program in terminal");
        return false;
    }
    
    io::stdout().execute(terminal::SetSize(S_WIDTH, S_HEIGHT)).unwrap();


    term.clear_screen().unwrap();
    term.hide_cursor().unwrap();

    true
}

pub fn clear() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

pub fn read_char() -> char {
    let term = Term::stdout();
    if let Ok(c) = term.read_char() {
        return c;
    } else {
        return ' ';
    }
}

pub fn draw() -> bool {
    let term = Term::stdout();
    //    let mut buffer = String::new();
    //    let mut buffer: Vec<(u16, u16, String)> = Vec::new();
    let mut buffer: Vec<(usize, usize, String)> = Vec::new();

    

    let (_sh, _sw) = term.size();
    if _sw < S_WIDTH {
        eprintln!("Terminal detection problem :: please run the program in terminal");
        return false;

    }

    let style: Style = Style::new();

    let sw: usize = _sw.into();
    let sh: usize = _sh.into();

    let mut width = 0;
    let mut height = 0;

    let mut _cxr = 0;
    let mut _cyb = 20;

    let mut hx = 0;
    let mut hy = 0;

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level") {
        for y in (0..20) {
            let mut cxr = 0;
            for x in (0..=29).rev() {
                if level[y][x] != ' ' && cxr == 0 {
                    cxr = x;
                    break;
                }
            }

            if cxr != 0 {
                if _cxr < cxr && cxr != 0 {
                    _cxr = cxr;
                }
            } else {
                if _cyb > y {
                    _cyb = y;
                }
            }
        }

        term.move_cursor_to(1, 35).unwrap();
        width = _cxr;
        width = ((sw / 5) / 2) - (width / 2);
        width = width * 5;
        height = _cyb;
        height = ((sh / 2) / 2) - (height / 2);
        height = height * 2;

        //-----------------
        for y in (0..20) {
            if y > _cyb {
                break;
            }

            for x in (0..30) {
                if x > _cxr {
                    break;
                }

                if level[y][x] == '@' {
                    hx = x;
                    hy = y;
                    break;
                }
            }
        }
        //   }
        //---Here clevel no owners and wrap
        //   let mut map: Level = *level;
        let mut map: Level = get_floor_map(hy, hx, 0, *level);
        let mut smap: Level = get_floor_map(hy, hx, 1, *level);
        let mut bmap: Level = get_floor_map(hy, hx, 2, *level);

        let pixels = image1::get();
         let ls = S_WIDTH * 3;
        //-----------------
        for y in (0..20) {
            if y > _cyb {
                break;
            }

            for x in (0..30) {
                if x > _cxr {
                    break;
                }

                let cell = level[y][x];
                let sx = x * 5 + width;
                let sy = y * 2 + height;

                if cell == '#'
                //Wall
                {
//-------------
/*
                let mut offset: usize = get_image_offset(y, x);
    	let fc: RGB8 = RGB8 {
	    r: pixels[offset+0] << 1,
	    g: pixels[offset+1] << 1 ,
	    b: pixels[offset+2] << 1,
	    };

    	let bc: RGB8 = RGB8 {
	    r: pixels[offset+0 + usize::from(ls)],
	    g: pixels[offset+1 + usize::from(ls)]  ,
	    b: pixels[offset+2 + usize::from(ls)] ,
	    };

    
    let style: Style = Style::new()
        .fg(Color::Color256(ansi256_from_rgb(fc)))
        .bg(Color::Color256(ansi256_from_rgb(bc)));

*/
                    let style = get_style(F_WALL_COLOR, B_WALL_COLOR);

                    buffer.push((
                        sx,
                        sy,
                        style

                            .apply_to("\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}")


                            .to_string(),
                    ));
 
                    buffer.push((
                        sx,
                        sy + 1,
                        style
                            .apply_to("\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}")
                            .to_string(),
                    ));
                } else if cell == '.'
                //Base
                {
                    let style = get_style(F_BASE_COLOR, B_BASE_COLOR);

                    buffer.push((
                        sx,
                        sy,
                        style.apply_to(" \u{250C}\u{2500}\u{2510} ").to_string(),
                    ));
                    buffer.push((
                        sx,
                        sy + 1,
                        style.apply_to(" \u{2514}\u{2500}\u{2518} ").to_string(),
                    ));
                } else if cell == '$'
                //Box
                {
                    if bmap[y][x] == 'l' {
                        let style = get_style(F_SBLOCK_COLOR, B_SBLOCK_COLOR);
                        buffer.push((
                            sx,
                            sy,
                            //style.apply_to(" \u{2554}\u{2550}\u{2557} ").to_string(),
                            style.apply_to(" \u{2554}\u{2550}\u{2557} ").to_string(),
                        ));
                        buffer.push((
                            sx,
                            sy + 1,
                            style.apply_to(" \u{255A}\u{2550}\u{255D} ").to_string(),
                        ));
                    } else {
                        let style = get_style(F_BLOCK_COLOR, B_BLOCK_COLOR);
                        buffer.push((
                            sx,
                            sy,
                            style.apply_to(" \u{2554}\u{2550}\u{2557} ").to_string(),
                        ));
                        buffer.push((
                            sx,
                            sy + 1,
                            style.apply_to(" \u{255A}\u{2550}\u{255D} ").to_string(),
                        ));
                    }
                //          buffer.push((sx, sy, style.apply_to(" \u{2554}\u{2550}\u{2557} ").to_string(),));
                //           buffer.push((sx, sy+1, style.apply_to(" \u{255A}\u{2550}\u{255D} ").to_string(),));
                } else if cell == '@'
                //Hero
                {
                    let style = get_style(F_HERO_COLOR, B_HERO_COLOR);
                    buffer.push((sx, sy, style.apply_to(" @@@ ").to_string()));
                    buffer.push((sx, sy + 1, style.apply_to(" @@@ ").to_string()));
                } else
                //Default space
                {
                    //let style = get_style(F_LEVEL_COLOR, B_LEVEL_COLOR);

                    if smap[y][x] == 'l' {
                        let style = get_style(F_SFLOOR_COLOR, B_SFLOOR_COLOR);
                        buffer.push((sx, sy, style.apply_to("     ").to_string()));
                        buffer.push((sx, sy + 1, style.apply_to("     ").to_string()));
                    } else if map[y][x] == 'l' {
                        let style = get_style(F_FLOOR_COLOR, B_FLOOR_COLOR);
                        buffer.push((sx, sy, style.apply_to("     ").to_string()));
                        buffer.push((sx, sy + 1, style.apply_to("     ").to_string()));
                    }
                }
                //----
            }
        }

        for (x, y, text) in buffer {
            if x > sw {
                continue;
            }

            if y > sh {
                continue;
            }

            term.move_cursor_to(x, y).unwrap();
            term.write_line(&text).unwrap();
        }
    }
    true
}

fn get_image_offset(y: usize, x: usize) -> usize {
     y * usize::from(S_WIDTH) * 3 + x * 3 
}

fn get_floor_map(hy: usize, hx: usize, is_flag: usize, mut map: Level) -> Level {
    let builder = thread::Builder::new().stack_size(10 * 1024 * 1024);
    let handle = builder
        .spawn(move || fill_level(hy, hx, is_flag, map))
        .unwrap();

    match handle.join() {
        Ok(result) => return result,
        Err(e) => eprintln!("Error: {:?}", e),
    }
    map
}

fn is_floor(c: char, is_flag: usize) -> bool {
    if is_flag == 0 {
        if c != '#' && /*c != '.' &&*/ c != 'l' {
            true
        } else {
            false
        }
    } else {
        if is_flag == 1 {
            if c == ' ' {
                true
            } else {
                false
            }
        } else {
            if c == '$' {
                true
            } else {
                false
            }
        }
    }
}

fn fill_level(hy: usize, hx: usize, is_flag: usize, mut map: Level) -> Level {
    if is_floor(map[hy][hx], is_flag) {
        map[hy][hx] = 'l';
    }

    if hy > 0 {
        let _hy = hy - 1;
        if is_floor(map[_hy][hx], is_flag) {
            map = fill_level(_hy, hx, is_flag, map);
        }
    }

    if hy < 19 {
        let _hy = hy + 1;
        if is_floor(map[_hy][hx], is_flag) {
            map = fill_level(_hy, hx, is_flag, map);
        }
    }

    if hx > 0 {
        let _hx = hx - 1;
        if is_floor(map[hy][_hx], is_flag) {
            map = fill_level(hy, _hx, is_flag, map);
        }
    }

    if hx < 29 {
        let _hx = hx + 1;
        if is_floor(map[hy][_hx], is_flag) {
            map = fill_level(hy, _hx, is_flag, map);
        }
    }

    map
}
