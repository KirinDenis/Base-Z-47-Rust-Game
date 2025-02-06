use crate::levels::CLEVEL;
use crate::levels::CURRENT_LEVEL_NAME;
use crate::levels::OLEVEL;

use crate::levels::level_const::BASE_CODE;
use crate::levels::level_const::BOX_CODE;
use crate::levels::level_const::FLOOR_CODE;
use crate::levels::level_const::HERO_CODE;
use crate::levels::level_const::L_HEIGHT;
use crate::levels::level_const::L_WIDTH;

pub const NO_STEP: usize = 0;
pub const CAN_STEP: usize = 1;
pub const NEXT_LEVEL: usize = 2;

struct Position {
    row: usize,
    col: usize,
}

fn get_hero_pos() -> Position {
    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut(CURRENT_LEVEL_NAME) {
        let mut _row = 0;
        let mut _col = 0;

        for row in level.iter() {
            for &cell in row.iter() {
                if cell == HERO_CODE
                //Wall
                {
                    return Position {
                        row: _row,
                        col: _col,
                    };
                }
                _col += 1;
            }
            _col = 0;
            _row += 1;
        }
    }
    return Position { row: 0, col: 0 };
}

fn update_pos(row: isize, col: isize, mut pos: Position) -> Position {
    if row > 0 {
        pos.row += 1;
    } else if row < 0 {
        pos.row -= 1;
    }

    if col > 0 {
        pos.col += 1;
    } else if col < 0 {
        pos.col -= 1;
    }
    return pos;
}

fn can_step(row: isize, col: isize) -> bool {
    let mut pos = get_hero_pos();

    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(level) = clevel.get_mut(CURRENT_LEVEL_NAME) {
        pos = update_pos(row, col, pos);

        if level[pos.row][pos.col] == FLOOR_CODE || level[pos.row][pos.col] == BASE_CODE {
            return true;
        } else if level[pos.row][pos.col] == BOX_CODE {
            pos = update_pos(row, col, pos);
            if level[pos.row][pos.col] == FLOOR_CODE || level[pos.row][pos.col] == BASE_CODE {
                level[pos.row][pos.col] = BOX_CODE;
                return true;
            }
        }
    }
    return false;
}

pub fn do_step(row: isize, col: isize) -> usize {
    let mut step_result = NO_STEP;
    if can_step(row, col) {
        let mut hero_pos = get_hero_pos();

        let mut clevel = CLEVEL.lock().unwrap();
        if let Some(clevel) = clevel.get_mut(CURRENT_LEVEL_NAME) {
            let mut olevel = OLEVEL.lock().unwrap();
            if let Some(olevel) = olevel.get_mut("original_level") {
                if olevel[hero_pos.row][hero_pos.col] != '$' {
                    clevel[hero_pos.row][hero_pos.col] = olevel[hero_pos.row][hero_pos.col];
                }
            }

            hero_pos = update_pos(row, col, hero_pos);

            clevel[hero_pos.row][hero_pos.col] = HERO_CODE;

            step_result = CAN_STEP;
        }
    }

    if check_win() {
        step_result = NEXT_LEVEL;
    }

    step_result
}

fn check_win() -> bool {
    let mut clevel = CLEVEL.lock().unwrap();
    if let Some(clevel) = clevel.get_mut(CURRENT_LEVEL_NAME) {
        let mut olevel = OLEVEL.lock().unwrap();
        if let Some(olevel) = olevel.get_mut("original_level") {
            for y in 0..L_HEIGHT {
                for x in 0..L_WIDTH {
                    if clevel[y][x] == BOX_CODE && olevel[y][x] != BASE_CODE {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}
