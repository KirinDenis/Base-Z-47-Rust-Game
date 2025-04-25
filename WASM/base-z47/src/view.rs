pub mod image0;
pub mod image1;
pub mod image2;
pub mod image3;
pub mod image4;


use hex::encode;

use crate::levels::level_const::WALL_CODE;
use crate::levels::load_level;
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

#[rustfmt::skip]

//pub const F_BASE_COLOR: &str = "rgb(0xC2, 0x14,  0x60)";
pub const F_FLOOR_COLOR: &str = "rgb(216, 253,  184)"; 
pub const F_SFLOOR_COLOR: &str = "rgb(190, 255,  125)"; 
pub const F_HERO_COLOR: &str = "rgb(255, 60, 140)"; 
pub const F_BLOCK_COLOR: &str = "rgb(80, 100, 200)"; 
pub const F_SBLOCK_COLOR: &str = "rgb(100, 100, 200)"; 
pub const F_BASE_COLOR: &str = "rgb(255, 220,  120)";
pub const F_WALL_COLOR: &str = "rgb(85, 85, 255)";
pub const B_SELECTED_COLOR: &str = "rgb(255, 85, 85)";

use wasm_bindgen::prelude::*;
use web_sys::{
    window, CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, KeyboardEvent,
};

pub fn draw_squares(ctx: &CanvasRenderingContext2d, width: u32, height: u32) {
    use js_sys::Math::random;

    let square_size = 20;
    let cols = width / square_size;
    let rows = height / square_size;

    for x in 0..cols {
        for y in 0..rows {
            let r = (random() * 255.0) as u8;
            let g = (random() * 255.0) as u8;
            let b = (random() * 255.0) as u8;
            let color = format!("rgb({}, {}, {})", r, g, b);
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(
                (x * square_size) as f64,
                (y * square_size) as f64,
                square_size as f64,
                square_size as f64,
            );
        }
    }
}


pub fn custom_draw(
    ctx: &CanvasRenderingContext2d,
    mut _width: usize,
    mut _height: usize,    
    offset_x: usize,
    offset_y: usize,
    level_number: usize,
    small: bool,
    selected: bool,
) -> bool {

    let _sh = 60;
    let _sw = 200;
    
   _width *= 5;

   let square_size = (_width) / _sw;
  //let square_size = 40;

    let mut _count_x_rigth = 0;
    let mut _count_y_buttom = 20;

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut("current_level")
     {
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
        width = ((_width / 5) / 2) - (width / 2);
        //width = width * 5;


        if _count_y_buttom > L_HEIGHT {
            _count_y_buttom = L_HEIGHT;
        }

        let mut height = _count_y_buttom;
        height = (_height  / 2) - (height * square_size/ 2);
        // height = height * 2;
        web_sys::console::log_1(&height.to_string().into());

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
                if small {
                    sx = x * 2;
                    sy = y;
                } else {
                    sx = x * square_size + width;
                    sy = y * square_size + height;
                }
                //web_sys::console::log_1(&sy.to_string().into());

                if cell == WALL_CODE
                //Wall
                {

                    let color = F_WALL_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (sx) as f64,
                        (sy) as f64,
                        square_size as f64,
                        square_size as f64,
                    );
        
                    /* 
                    let style = get_style(F_WALL_COLOR, B_WALL_COLOR);

                    if small {
                        buffer.push((sx, sy, style.apply_to(WALL_DRAW_SMALL).to_string()));
                    } else {
                        buffer.push((sx, sy, style.apply_to(WALL_DRAW_UP).to_string()));
                        buffer.push((sx, sy + 1, style.apply_to(WALL_DRAW_DN).to_string()));
                    }
                    */
                } else if cell == BASE_CODE
                //Base
                {
                    let color = F_BASE_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (x * square_size) as f64,
                        (y  * square_size) as f64,
                        square_size as f64,
                        square_size as f64,
                    );

                    /* 
                    let style = get_style(F_BASE_COLOR, B_BASE_COLOR);

                    if small {
                        buffer.push((sx, sy, style.apply_to(BASE_DRAW_SMALL).to_string()));
                    } else {
                        buffer.push((sx, sy, style.apply_to(BASE_DRAW_UP).to_string()));
                        buffer.push((sx, sy + 1, style.apply_to(BASE_DRAW_DN).to_string()));
                    }
                    */
                }                
                 else if cell == BOX_CODE
                //Box
                {
                    if bmap[y][x] == LEVEL_CODE {
			/*
                        let style = get_style(F_SBLOCK_COLOR, B_SBLOCK_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(BOX_DRAW_DN).to_string()));
                        }
			*/
                    let color = F_SBLOCK_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (x * square_size) as f64,
                        (y  * square_size) as f64,
                        square_size as f64,
                        square_size as f64,
                    );


                    } else {
			/*
                        let style = get_style(F_BLOCK_COLOR, B_BLOCK_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(BOX_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(BOX_DRAW_DN).to_string()));
                        }
			*/
                    let color = F_BLOCK_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (x * square_size) as f64,
                        (y  * square_size) as f64,
                        square_size as f64,
                        square_size as f64,
                    );

                    }
                } else if cell == HERO_CODE
                //Hero
                {
		/*
                    let style = get_style(F_HERO_COLOR, B_HERO_COLOR);

                    if small {
                        buffer.push((sx, sy, style.apply_to(HERO_DRAW_SMALL).to_string()));
                    } else {
                        buffer.push((sx, sy, style.apply_to(HERO_DRAW_UP).to_string()));
                        buffer.push((sx, sy + 1, style.apply_to(HERO_DRAW_DN).to_string()));
                    }
		*/
                    let color = F_HERO_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (x * square_size) as f64,
                        (y  * square_size) as f64,
                        square_size as f64,
                        square_size as f64,
                    );

                } else
                //Default space
                {
                   if smap[y][x] == LEVEL_CODE  {
			/*
                        let style = get_style(F_SFLOOR_COLOR, B_SFLOOR_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(FLOOR_DRAW_DN).to_string()));
                        }
			*/
                    let color =F_SFLOOR_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (x * square_size) as f64,
                        (y  * square_size) as f64,
                        square_size as f64,
                        square_size as f64,
                    );

                    } else if map[y][x] == LEVEL_CODE {
			/*
                        let style = get_style(F_FLOOR_COLOR, B_FLOOR_COLOR);

                        if small {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_SMALL).to_string()));
                        } else {
                            buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_UP).to_string()));
                            buffer.push((sx, sy + 1, style.apply_to(FLOOR_DRAW_DN).to_string()));
                        }
			*/
                    let color =F_FLOOR_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (x * square_size) as f64,
                        (y  * square_size) as f64,
                        square_size as f64,
                        square_size as f64,
                    );

                    } else if selected {
                        //let style = get_style(F_FLOOR_COLOR, B_SELECTED_COLOR);
                        //buffer.push((sx, sy, style.apply_to(FLOOR_DRAW_SMALL).to_string()));

                    let color =F_SFLOOR_COLOR;
                    ctx.set_fill_style(&color.into());
                    ctx.fill_rect(
                        (x * square_size) as f64,
                        (y  * square_size) as f64,
                        square_size as f64,
                        square_size as f64,
                    );

                    }
                }             
            }
                
        }

        /*
        let mut _offset_y = offset_y;
        if small {
            _offset_y = _offset_y + (L_HEIGHT / 2 - _count_y_buttom / 2);
        }

        let mut _offset_x = offset_x;
        if small {
            _offset_x = _offset_x + (L_WIDTH / 2 - _count_x_rigth / 2);
        }

        for (x, y, text) in buffer {
            if x > screen_width {
                continue;
            }

            if y > screen_height {
                continue;
            }

            term.move_cursor_to(x + _offset_x, y + _offset_y).unwrap();

            term.write_line(&text).unwrap();
        }

        if small && selected {
            let s_height: usize = (S_HEIGHT).into();
            term.move_cursor_to(0, s_height - 1).unwrap();

            let style = get_style(B_FLOOR_COLOR, B_SELECTED_COLOR);
            for _x in 0..S_WIDTH {
                print!("{}", style.apply_to(" "));
            }
            term.move_cursor_to(0, s_height - 1).unwrap();
            print!("{}", style.apply_to(&format!("Level {}", level_number)));
            print!("{}", style.apply_to(" :: use row keys for select/move level, Enter/Space to play, ESC to quit, '+/-/p' for music control :: "));
        }
    }
     */
    }
    true
}

fn get_floor_map(hy: usize, hx: usize, is_flag: usize, map: Level) -> Level {

    /*
    let builder = thread::Builder::new().stack_size(10 * 1024 * 1024);
    let handle = builder
        .spawn(move || fill_level(hy, hx, is_flag, map))
        .unwrap();
 */
    fill_level(hy, hx, is_flag, map)
    /*
    match handle.join() {
        Ok(result) => return result,
        Err(e) => eprintln!("Error: {:?}", e),
    }
     */
   // map
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


pub fn intro_image(ctx: &CanvasRenderingContext2d, width: u32, height: u32, image: usize) {
    for i in (0..8).rev() {
        draw_image_ex(ctx, width, height, image, i);
        // thread::sleep(Duration::from_millis(100));
    }
}

pub fn draw_image(ctx: &CanvasRenderingContext2d, width: u32, height: u32, image: usize) {
    draw_image_ex(ctx, width, height, image, 0);
}

#[wasm_bindgen]
pub fn draw_image_ex(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    image: usize,
    hide: u8,
) {
    let _sh = 60;
    let _sw = 200;

    let square_size = width / _sw;

    let pixels;

    if image == 0 {
        pixels = image0::get();
    } else if image == 1 {
        pixels = image1::get();
    } else if image == 2 {
        pixels = image2::get();
    } else if image == 3 {
        pixels = image3::get();
    } else {
        pixels = image4::get();
    }

    let psize = 74000;
    let lsize = 200 * 3;

    let mut count: usize = 0;

    let mut x = 0;
    let mut y = 0;

    for i in (0..psize).step_by(lsize * 2) {
        for j in (i..i + lsize).step_by(3) {
            let r = (pixels[j + 0]) as u8;
            let g = (pixels[j + 1]) as u8;
            let b = (pixels[j + 2]) as u8;
            let color = format!("rgb({}, {}, {})", r, g, b);
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(
                (x * square_size) as f64,
                (y * square_size) as f64,
                square_size as f64,
                square_size as f64,
            );

            let r = (pixels[j + 0 + lsize]) as u8;
            let g = (pixels[j + 1 + lsize]) as u8;
            let b = (pixels[j + 2 + lsize]) as u8;
            let color = format!("rgb({}, {}, {})", r, g, b);
            ctx.set_fill_style(&color.into());
            ctx.fill_rect(
                (x * square_size) as f64,
                ((y + 1) * square_size) as f64,
                square_size as f64,
                square_size as f64,
            );
            x = x + 1;
            count = count + 1;
        }
        x = 0;
        y = y + 2;
    }
}
