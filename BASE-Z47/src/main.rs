// Historical Note:
/*
Sokoban, a classic puzzle game developed in 1981 by Hiroyuki Imabayashi, challenges players to push boxes to designated locations within a warehouse maze.

Sokoban, a Japanese term meaning "warehouse keeper," is a classic puzzle game that has intrigued players and researchers alike since its inception. Designed by Hiroyuki Imabayashi in 1981, the game was first published by Thinking Rabbit in December 1982.
https://en.wikipedia.org/wiki/Sokoban


In Sokoban, players navigate through a warehouse, pushing boxes onto designated storage locations. The challenge lies in the game's simplicity combined with its deep strategic elements; players can only push boxes (not pull), and a box pushed into a corner renders it immovable, potentially making the puzzle unsolvable.

The game's elegant yet demanding nature has made it a subject of extensive scientific research, particularly in the field of computational complexity. Solving Sokoban puzzles has been proven to be NP-hard, and further studies have classified it as PSPACE-complete.

This means that determining a solution is computationally intensive, with the difficulty increasing exponentially with the size of the puzzle.

Several factors contribute to the complexity of solving Sokoban puzzles:

    High Branching Factor: At any given state, there are numerous possible moves, leading to a vast decision tree that algorithms must navigate.

    Large Search Depth: Some puzzles require a long sequence of moves to reach the solution, demanding algorithms to plan many steps ahead.

    Irreversible Actions: Pushing a box into a corner or against a wall can render the puzzle unsolvable, necessitating careful planning to avoid such deadlocks.

These challenges have made Sokoban a benchmark for evaluating and developing artificial intelligence and automated planning systems. Early automated solvers, like Rolling Stone developed at the University of Alberta, utilized conventional search algorithms enhanced with domain-specific knowledge. Despite advancements, many complex Sokoban puzzles remain unsolvable by computers, highlighting the game's intricate complexity.


For those interested in the mathematical and logical foundations of Sokoban, several resources delve into its computational aspects:

    "Sokoban is PSPACE-complete" by Joseph C. Culberson: This technical report provides an in-depth analysis of the game's computational classification.

    "Sokoban: Enhancing general single-agent search methods using domain knowledge": This paper discusses strategies for improving search algorithms tailored to Sokoban's unique challenges.

Sokoban's enduring appeal lies in its perfect blend of straightforward rules and profound strategic depth, offering both casual players and researchers a rich domain for exploration and challenge.

*/


mod backgroundthread;
mod levels;
mod model;
mod sound;
mod view;

// Background sound dependencies
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use crate::model::do_step;
use crossterm::event::KeyCode;

// Key bindings for movement and controls
const UP_KEY: KeyCode = KeyCode::Up;
const DOWN_KEY: KeyCode = KeyCode::Down;
const LEFT_KEY: KeyCode = KeyCode::Left;
const RIGHT_KEY: KeyCode = KeyCode::Right;

const SELECT_KEY: KeyCode = KeyCode::Enter;
const QUIT_KEY: KeyCode = KeyCode::Esc;

// Function to display level selection screen
fn select_level(levelindex: usize) {
    view::draw_image(0); // Draw background image
    let mut x_offset = 10;
    let y_offset = 25;
    
    for i in levelindex..levelindex + 3 {
        levels::load_level(&format!("level{}", i)); // Load the level data
        view::custom_draw(x_offset, y_offset, i, true, i == levelindex); // Draw the level preview
        x_offset += 60;
    } 
}

fn main() {
    // Initialize the game controller, exit if initialization fails
    if !view::init() {
        return;
    }
    view::intro_image(0); // Display intro image

    let mut mode: usize = 0; // 0 = Level selection, 1 = Game mode
    let mut levelindex = 1;
    let mut step_result: usize = model::NO_STEP;
    select_level(levelindex);

    // Background music setup
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&handle).unwrap()));

    let files = vec!["assets/music.mp3", "assets/music.mp3"];
    for file in &files {
        let file = BufReader::new(File::open(file).unwrap());
        let source = Decoder::new(file).unwrap().repeat_infinite();
        sink.lock().unwrap().append(source);
    }
    
    let sink = sink.lock().unwrap();
    sink.set_volume(0.1); // Set initial volume
    
    // Main game loop
    loop {
        let key = view::read_char();

        // Ignore specific key input
        if key == KeyCode::Char('_') {
            continue;
        }

        // Handle background music controls
        if key == KeyCode::Char('p') {
            if sink.is_paused() {
                sink.play(); // Resume music
            } else {
                sink.pause(); // Pause music
            }
        } else if key == KeyCode::Char('+') {
            let vol = sink.volume() + 0.1;
            sink.set_volume(vol.max(1.0)); // Increase volume
        } else if key == KeyCode::Char('-') {
            let vol = sink.volume() - 0.1;
            sink.set_volume(vol.max(0.0)); // Decrease volume
        }

        // Handle level selection mode
        if mode == 0 {
            if key == LEFT_KEY {
                levelindex -= 1;
                select_level(levelindex);
            } else if key == RIGHT_KEY {
                levelindex += 1;
                select_level(levelindex);
            } else if key == SELECT_KEY || key == KeyCode::Char(' ') {
                mode = 1;
                view::set_level(levelindex); // Start game with selected level
            } else if key == QUIT_KEY {            
                break; // Exit game
            }
        } else { // Handle gameplay mode
            if key == KeyCode::Char('1') {
                levelindex -= 1;
                view::set_level(levelindex);
            } else if key == KeyCode::Char('2') {
                levelindex += 1;
                view::set_level(levelindex);
            } else if key == UP_KEY {
                step_result = do_step(-1, 0); // Move up
            } else if key == DOWN_KEY {
                step_result = do_step(1, 0); // Move down
            } else if key == LEFT_KEY {
                step_result = do_step(0, -1); // Move left
            } else if key == RIGHT_KEY {
                step_result = do_step(0, 1); // Move right
            } 
            
            if key == QUIT_KEY {            
                mode = 0;
                select_level(levelindex); // Return to level selection
            }

            // Process movement results
            if step_result == model::NO_STEP {
                // No movement
            } else if step_result == model::CAN_STEP {
                sound::step_sound(); // Play step sound
                if !view::draw() {
                    break; // Exit if rendering fails
                }
            } else if step_result == model::NEXT_LEVEL {
                levelindex += 1;
                view::set_level(levelindex); // Load next level
            }

            step_result = 0;
        }
    }
}
