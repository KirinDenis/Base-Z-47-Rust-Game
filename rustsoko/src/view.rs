use ansi_colours::*;
use console::Color;
use console::Style;
use console::Term;
use crossterm::{terminal, ExecutableCommand};
use rgb::RGB8;
use std::io::{self};
use std::thread;

use crate::levels::level_const::WALL_CODE;
use crate::levels::Level;
use crate::levels::CLEVEL;

use crate::levels::level_const::L_HEIGHT;
use crate::levels::level_const::L_WIDTH;
use crate::levels::level_const::S_HEIGHT;
use crate::levels::level_const::S_WIDTH;

use crate::levels::level_const::BASE_CODE;
use crate::levels::level_const::BOX_CODE;
use crate::levels::level_const::FLOOR_CODE;
use crate::levels::level_const::HERO_CODE;

const LEVEL_CODE: char = 'l';

const IS_LEVEL_MAP: usize = 0;
const IS_FLOOR_MAP: usize = 1;
const IS_BOX_MAP: usize = 2;

//Drawing game object 5 char width 2 char height
const WALL_DRAW_UP: &str = "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}";
const WALL_DRAW_DN: &str = "\u{2588}\u{2588}\u{2588}\u{2588}\u{2588}";

const WALL_DRAW_SMALL: &str = "\u{2588}\u{2588}";

const BASE_DRAW_UP: &str = " \u{250C}\u{2500}\u{2510} ";
const BASE_DRAW_DN: &str = " \u{2514}\u{2500}\u{2518} ";

const BASE_DRAW_SMALL: &str = "  ";

const BOX_DRAW_UP: &str = " \u{2554}\u{2550}\u{2557} ";
const BOX_DRAW_DN: &str = " \u{255A}\u{2550}\u{255D} ";

const BOX_DRAW_SMALL: &str = "  ";

const HERO_DRAW_UP: &str = "\u{2571}\u{2554}\u{2500}\u{2557}\u{2572}";
const HERO_DRAW_DN: &str = "\u{2572}\u{255A}\u{2500}\u{255D}\u{2571}";

const HERO_DRAW_SMALL: &str = "\u{2571}\u{2572}";

const FLOOR_DRAW_UP: &str = "     ";
const FLOOR_DRAW_DN: &str = "     ";

const FLOOR_DRAW_SMALL: &str = "  ";

#[rustfmt::skip]
pub const F_FLOOR_COLOR: RGB8 = RGB8 {r: 0xF7, g: 0xF0, b: 0xD4,};

#[rustfmt::skip]
pub const B_FLOOR_COLOR: RGB8 = RGB8 {r: 0xE7, g: 0xE0, b: 0xC4,};

#[rustfmt::skip]
pub const F_SFLOOR_COLOR: RGB8 = RGB8 {r: 0xF7, g: 0xF0, b: 0xD4,};

#[rustfmt::skip]
pub const B_SFLOOR_COLOR: RGB8 = RGB8 {r: 0xF7, g: 0xF0, b: 0xD4,};

#[rustfmt::skip]
pub const F_HERO_COLOR: RGB8 = RGB8 {r: 0xF7, g: 0xF0, b: 0xD4,};

#[rustfmt::skip]
pub const B_HERO_COLOR: RGB8 = RGB8 {r: 0xC2, g: 0x14, b: 0x60,};

#[rustfmt::skip]
pub const F_BLOCK_COLOR: RGB8 = RGB8 {r: 0x34, g: 0x7B, b: 0x98,};

#[rustfmt::skip]
pub const B_BLOCK_COLOR: RGB8 = RGB8 {r: 0xFC, g: 0xCB, b: 0x1A,};

#[rustfmt::skip]
pub const F_SBLOCK_COLOR: RGB8 = RGB8 {r: 0x34, g: 0x7B, b: 0x98,};

#[rustfmt::skip]
pub const B_SBLOCK_COLOR: RGB8 = RGB8 {r: 0xEC, g: 0xBB, b: 0x0A,};

#[rustfmt::skip]
pub const F_BASE_COLOR: RGB8 = RGB8 {r: 0xC2, g: 0x14, b: 0x60,};

#[rustfmt::skip]
pub const B_BASE_COLOR: RGB8 = RGB8 {r: 0xF7, g: 0xF0, b: 0xD4,};

#[rustfmt::skip]
pub const F_WALL_COLOR: RGB8 = RGB8 {r: 0x34, g: 0x7B, b: 0x98,};

#[rustfmt::skip]
pub const B_WALL_COLOR: RGB8 = RGB8 {r: 0x34, g: 0x7B, b: 0x98,};

fn get_style(foreground: RGB8, background: RGB8) -> Style {
    Style::new()
        .fg(Color::Color256(ansi256_from_rgb(foreground)))
        .bg(Color::Color256(ansi256_from_rgb(background)))
}

pub fn init() -> bool {
    let term = Term::stdout();

    if !term.is_term() {
        eprintln!("Terminal detection problem :: please run the program in terminal");
        return false;
    }

    io::stdout()
        .execute(terminal::SetSize(S_WIDTH, S_HEIGHT))
        .unwrap();

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
        return FLOOR_CODE;
    }
}

pub fn draw() -> bool {
    custom_draw(0, 0, false)
}

pub fn custom_draw(offset_x: usize, offset_y: usize, small: bool) -> bool {
    let term = Term::stdout();

    let mut buffer: Vec<(usize, usize, String)> = Vec::new();

    let (_screen_height, _screen_width) = term.size();
    if _screen_width < S_WIDTH {
        eprintln!("Terminal detection problem :: please run the program in terminal");
        return false;
    }

    let screen_height: usize = _screen_height.into();
    let screen_width: usize = _screen_width.into();

    let mut _count_x_rigth = 0;
    let mut _count_y_buttom = 20;

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level") {
        for y in 0..20 {
            let mut cxr = 0;
            for x in (0..=29).rev() {
                if level[y][x] != FLOOR_CODE && cxr == 0 {
                    cxr = x;
                    break;
                }
            }

            if cxr != 0 {
                if _count_x_rigth < cxr && cxr != 0 {
                    _count_x_rigth = cxr;
                }
            } else {
                if _count_y_buttom > y {
                    _count_y_buttom = y;
                }
            }
        }

        let mut width = _count_x_rigth;
        //Drawing game object 5 char width 2 char height
        width = ((screen_width / 5) / 2) - (width / 2);
        width = width * 5;

        if _count_y_buttom > L_HEIGHT {
            _count_y_buttom = L_HEIGHT;
        }

        let mut height = _count_y_buttom;
        height = ((screen_height / 2) / 2) - (height / 2);
        height = height * 2;

        let mut hero_x = 0;
        let mut hero_y = 0;

        for y in 0.._count_y_buttom {
            for x in 0.._count_x_rigth {
                if level[y][x] == HERO_CODE {
                    hero_x = x;
                    hero_y = y;
                    break;
                }
            }
        }

        _count_x_rigth = _count_x_rigth + 1;
        //---Here clevel no owners and wrap
        //   let mut map: Level = *level;
        let map: Level = get_floor_map(hero_y, hero_x, IS_LEVEL_MAP, *level);
        let smap: Level = get_floor_map(hero_y, hero_x, IS_FLOOR_MAP, *level);
        let bmap: Level = get_floor_map(hero_y, hero_x, IS_BOX_MAP, *level);

        for y in 0.._count_y_buttom {
            for x in 0.._count_x_rigth {
                let cell = level[y][x];

                let sx: usize;
                let sy: usize;
                if small 
                {
                    sx = x * 2;
                    sy = y ;    
                } else {
                  sx = x * 5 + width;
                  sy = y * 2 + height;
                }

                if cell == WALL_CODE
                //Wall
                {
                    let style = get_style(F_WALL_COLOR, B_WALL_COLOR);

                    if small {
                        buffer.push((sx, sy, style.apply_to(WALL_DRAW_SMALL).to_string()));
                    } else {
                        buffer.push((sx, sy, style.apply_to(WALL_DRAW_UP).to_string()));
                        buffer.push((sx, sy + 1, style.apply_to(WALL_DRAW_DN).to_string()));
                    }
                } else if cell == BASE_CODE
                //Base
                {
                    let style = get_style(F_BASE_COLOR, B_BASE_COLOR);

                    if small {
                        buffer.push((sx, sy, style.apply_to(BASE_DRAW_SMALL).to_string()));
                    } else {
                        buffer.push((sx, sy, style.apply_to(BASE_DRAW_UP).to_string()));
                        buffer.push((sx, sy + 1, style.apply_to(BASE_DRAW_DN).to_string()));
                    }
                } else if cell == BOX_CODE
                //Box
                {
                    if bmap[y][x] == LEVEL_CODE {
                        let style = get_style(F_SBLOCK_COLOR, B_SBLOCK_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(BOX_DRAW_DN).to_string()));
                        }
                    } else {
                        let style = get_style(F_BLOCK_COLOR, B_BLOCK_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(BOX_DRAW_DN).to_string()));
                        }
                    }
                } else if cell == HERO_CODE
                //Hero
                {
                    let style = get_style(F_HERO_COLOR, B_HERO_COLOR);

                    if small {
                        buffer.push((sx, sy, style.apply_to(HERO_DRAW_SMALL).to_string()));
                    } else {
                        buffer.push((sx, sy, style.apply_to(HERO_DRAW_UP).to_string()));
                        buffer.push((sx, sy + 1, style.apply_to(HERO_DRAW_DN).to_string()));
                    }
                } else
                //Default space
                {
                    if smap[y][x] == LEVEL_CODE {
                        let style = get_style(F_SFLOOR_COLOR, B_SFLOOR_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(FLOOR_DRAW_DN).to_string()));
                        }
                    } else if map[y][x] == LEVEL_CODE {
                        let style = get_style(F_FLOOR_COLOR, B_FLOOR_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(FLOOR_DRAW_DN).to_string()));
                        }
                    }
                }
            }
        }

        for (x, y, text) in buffer {
            if x > screen_width {
                continue;
            }

            if y > screen_height {
                continue;
            }

            term.move_cursor_to(x + offset_x, y + offset_y).unwrap();
            term.write_line(&text).unwrap();
        }
    }
    true
}

fn get_floor_map(hy: usize, hx: usize, is_flag: usize, map: Level) -> Level {
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
    if is_flag == IS_LEVEL_MAP {
        if c != WALL_CODE && /*c != '.' &&*/ c != LEVEL_CODE {
            true
        } else {
            false
        }
    } else {
        if is_flag == IS_FLOOR_MAP {
            if c == FLOOR_CODE {
                true
            } else {
                false
            }
        } else {
            if is_flag == IS_BOX_MAP {
                if c == BOX_CODE {
                    true
                } else {
                    false
                }
            } else {
                //default wrong is_flag walue
                false
            }
        }
    }
}

fn fill_level(hy: usize, hx: usize, is_flag: usize, mut map: Level) -> Level {
    if is_floor(map[hy][hx], is_flag) {
        map[hy][hx] = LEVEL_CODE;
    }

    if hy > 0 {
        let _hy = hy - 1;
        if is_floor(map[_hy][hx], is_flag) {
            map = fill_level(_hy, hx, is_flag, map);
        }
    }

    if hy < L_HEIGHT - 1 {
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

    if hx < L_WIDTH - 1 {
        let _hx = hx + 1;
        if is_floor(map[hy][_hx], is_flag) {
            map = fill_level(hy, _hx, is_flag, map);
        }
    }

    map
}
