mod utils;

mod view;

mod levels;
mod model;

use crate::model::do_step;
use crate::levels::load_level;
use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, Document, HtmlCanvasElement, HtmlElement, KeyboardEvent,
};

use std::thread;
use std::time::Duration;

// Function to display level selection screen
fn select_level(levelindex: usize) {
//    view::draw_image(0); // Draw background image
    let mut x_offset = 10;
    let y_offset = 25;
    
    for i in levelindex..levelindex + 3 {
        levels::load_level(&format!("level{}", i)); // Load the level data
  //      view::custom_draw(x_offset, y_offset, i, true, i == levelindex); // Draw the level preview
        x_offset += 60;
    } 
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut imagecount = 0;
    let mut levelindex = 1;

    let document = window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;
    let body = document.body().unwrap();
    body.append_child(&canvas)?;

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    let context = Rc::new(RefCell::new(context));
    let canvas = Rc::new(canvas);

    // Resize canvas to full window size
    let context_clone = Rc::clone(&context);
    let canvas_clone = Rc::clone(&canvas);

    let closure = Closure::wrap(Box::new(move || {
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
        let height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
        canvas_clone.set_width(width);
        canvas_clone.set_height(height);
        

        load_level(&format!("level{}", levelindex));
        view::draw_image_ex(&context_clone.borrow(), width, height, imagecount, 0);        
        view::custom_draw(&context_clone.borrow(), width as usize, height as usize, 0 , 0, levelindex, false, false);
    
    }) as Box<dyn FnMut()>);

    window()
        .unwrap()
        .set_onresize(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    let width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
    let height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;

    canvas.set_width(width);
    canvas.set_height(height);

    let ctx = context.borrow();


    load_level(&format!("level{}", levelindex));
    view::draw_image_ex(&context.borrow(), width, height, imagecount, 0);
    view::custom_draw(&context.borrow(), width as usize, height as usize, 0 , 0, levelindex, false, false);


    let context_clone = Rc::clone(&context);
    let canvas_clone = Rc::clone(&canvas);


    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key = event.key().to_lowercase();

         web_sys::console::log_1(&key.as_str().into());

        if key == "enter" {
            imagecount = imagecount + 1;
            view::draw_image_ex(&context_clone.borrow(), width, height, imagecount, 0);
            view::custom_draw(&context_clone.borrow(), width as usize, height as usize, 0 , 0, levelindex, false, false);
        } else if key == "arrowup" {
            do_step(-1, 0);
           // levelindex += 1;
           // load_level(&format!("level{}", levelindex));
          //  view::draw_image_ex(&context_clone.borrow(), width, height, imagecount, 0);
            view::custom_draw(&context_clone.borrow(), width as usize, height as usize, 0 , 0, levelindex, false, false);

            // let mut text = content.text_content().unwrap_or_default();
            //text.pop();
            //content.set_text_content(Some(&text));
        } else if key.len() == 1 {


            //let mut text = content.text_content().unwrap_or_default();
            //text.push_str(&key);
            //content.set_text_content(Some(&text));
        }
    }) as Box<dyn FnMut(_)>);

    window()
        .unwrap()
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    Ok(())
}
