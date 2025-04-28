mod utils;
mod view;
mod levels;
mod model;

use crate::levels::load_level;
use crate::model::do_step;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent,
};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let mut imagecount = 3;
    let mut levelindex = 1;
    let mut width;
    let mut height;

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

        web_sys::console::log_1(&width.to_string().into());
        // load_level(&format!("level{}", levelindex));
        view::draw_image_ex(&context_clone.borrow(), width,  imagecount);
        view::custom_draw(
            &context_clone.borrow(),
            width as usize,
            height as usize,
            0,
            0,            
            false,
            false,
        );
    }) as Box<dyn FnMut()>);

    window()
        .unwrap()
        .set_onresize(Some(closure.as_ref().unchecked_ref()));

    closure.forget();
    
    width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
    height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
    canvas.set_width(width);
    canvas.set_height(height);

    load_level(&format!("level{}", levelindex));
    view::draw_image_ex(&context.borrow(), width,  imagecount);
    view::custom_draw(
        &context.borrow(),
        width as usize,
        height as usize,
        0,
        0,        
        false,
        false,
    );

    let context_clone = Rc::clone(&context);    
    let mut step_result: usize = model::WAIT_STEP;

    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key = event.key().to_lowercase();

        if window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32 != width {
            width = window().unwrap().inner_width().unwrap().as_f64().unwrap() as u32;
            height = window().unwrap().inner_height().unwrap().as_f64().unwrap() as u32;
            canvas.set_width(width);
            canvas.set_height(height);
            web_sys::console::log_1(&width.to_string().into());
            view::draw_image_ex(&context_clone.borrow(), width,  imagecount);
            view::custom_draw(
                &context_clone.borrow(),
                width as usize,
                height as usize,
                0,
                0,                
                false,
                false,
            );
        }

        if key == "enter" {
            event.prevent_default();
            imagecount = imagecount + 1;
            view::draw_image_ex(&context_clone.borrow(), width,  imagecount);
            view::custom_draw(
                &context_clone.borrow(),
                width as usize,
                height as usize,
                0,
                0,                
                false,
                false,
            );
        } else if key == "arrowup" {
            step_result = do_step(-1, 0);
        } else if key == "arrowdown" {
            step_result = do_step(1, 0);
        } else if key == "arrowleft" {
            step_result = do_step(0, -1);
        } else if key == "arrowright" {
            step_result = do_step(0, 1);
        } else if key == "1" {
            event.prevent_default();
            levelindex -= 1;
            load_level(&format!("level{}", levelindex));
            view::draw_image_ex(&context_clone.borrow(), width,  imagecount);
            view::custom_draw(
                &context_clone.borrow(),
                width as usize,
                height as usize,
                0,
                0,                
                false,
                false,
            );
        } else if key == "2" {
            event.prevent_default();
            levelindex += 1;
            load_level(&format!("level{}", levelindex));
            view::draw_image_ex(&context_clone.borrow(), width, imagecount);
            view::custom_draw(
                &context_clone.borrow(),
                width as usize,
                height as usize,
                0,
                0,                
                false,
                false,
            );
        }

        // Process movement results
        if step_result == model::NO_STEP {
            event.prevent_default();
        } else if step_result == model::CAN_STEP {
            //sound::step_sound(); // Play step sound
            event.prevent_default();
            view::custom_draw(
                &context_clone.borrow(),
                width as usize,
                height as usize,
                0,
                0,                
                false,
                false,
            );
        } else if step_result == model::NEXT_LEVEL {
            levelindex += 1;
            //view::set_level(levelindex); // Load next level
            load_level(&format!("level{}", levelindex));
            view::draw_image_ex(&context_clone.borrow(), width, imagecount);
            view::custom_draw(
                &context_clone.borrow(),
                width as usize,
                height as usize,
                0,
                0,                
                false,
                false,
            );
        }
        step_result = model::WAIT_STEP;
    }) as Box<dyn FnMut(_)>);

    window()
        .unwrap()
        .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())
        .unwrap();
    closure.forget();

    Ok(())
}
