mod utils;

mod view;


use std::rc::Rc;
use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, Document, HtmlElement, KeyboardEvent, HtmlCanvasElement, CanvasRenderingContext2d};

use std::thread;
use std::time::Duration;

fn draw_squares(ctx: &CanvasRenderingContext2d, width: u32, height: u32) {
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

fn init()-> Result<(), JsValue> {
    let document = window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
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
        draw_squares(&context_clone.borrow(), width, height);
    }) as Box<dyn FnMut()>);

    window().unwrap().set_onresize(Some(closure.as_ref().unchecked_ref()));
    closure.forget(); 

    // Initial drawing
    let width = canvas.width();
    
    let height = canvas.height();

    
    let ctx = context.borrow();
     draw_squares(&context.borrow(), width, height);    
     Ok(())
}


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {

    init();

    Ok(())
/*
    let console = document.create_element("div").unwrap();
    console.set_attribute("id", "console").unwrap();
    console.set_attribute("tabindex", "0").unwrap(); 
    document.body().unwrap().append_child(&console).unwrap();

    let pre = document.create_element("pre").unwrap();
    let code = document.create_element("code").unwrap();
    code.set_attribute("id", "code").unwrap();
    pre.append_child(&code); 

    let line = document.create_element("div").unwrap();
    line.set_attribute("class", "line").unwrap();
    code.append_child(&line); 

    let content = document.create_element("span").unwrap();
    content.set_attribute("class", "content").unwrap();
    line.append_child(&content).unwrap();


    let mut text = content.text_content().unwrap_or_default();
    text.push_str("Base-Z47");
    content.set_text_content(Some(&text));


    let cursor = document.create_element("span").unwrap();
    cursor.set_attribute("class", "cursor").unwrap();
    cursor.set_text_content(Some("_"));
    line.append_child(&cursor).unwrap();

    console.append_child(&pre).unwrap();

    let colors = ["red", "blue", "green", "purple", "brown", "black"];
    let mut text = code.text_content().unwrap_or_default();

    view::draw_image_ex(0,0);

    let mut imagecount = 0; 
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        let key = event.key();

        let document = window().unwrap().document().unwrap();
        let console = document.get_element_by_id("console").unwrap();
        let last_line = console.last_element_child().unwrap();
        let content = last_line.first_element_child().unwrap(); 
        let cursor = last_line.last_element_child().unwrap(); 

        if key == "Enter" {

            cursor.set_text_content(None); 
            let new_line = document.create_element("div").unwrap();
            new_line.set_attribute("class", "line").unwrap();

            let new_content = document.create_element("span").unwrap();
            new_content.set_attribute("class", "content").unwrap();
            new_line.append_child(&new_content).unwrap();

            let new_cursor = document.create_element("span").unwrap();
            new_cursor.set_attribute("class", "cursor").unwrap();
            new_cursor.set_text_content(Some("_"));
            new_line.append_child(&new_cursor).unwrap();

            console.append_child(&new_line).unwrap();

            imagecount = imagecount + 1;
            view::draw_image_ex(imagecount,0); 
        } else if key == "Backspace" {

            let mut text = content.text_content().unwrap_or_default();
            text.pop(); 
            content.set_text_content(Some(&text));
        } else if key.len() == 1 {

            let mut text = content.text_content().unwrap_or_default();
            text.push_str(&key);
            content.set_text_content(Some(&text));
           
        }


        console.scroll_top();
    }) as Box<dyn FnMut(_)>);

    console.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
*/
}
fn log_to_console(message: &str) {
    let document = window().unwrap().document().unwrap();
    let console = document.get_element_by_id("console").unwrap();
    let line = document.create_element("div").unwrap();
    line.set_text_content(Some(message));
    console.append_child(&line).unwrap();
}

fn append_to_console(input: &str) {
    let document = window().unwrap().document().unwrap();
    let console = document.get_element_by_id("console").unwrap();
    let current_line = document.create_element("span").unwrap();
    current_line.set_text_content(Some(input));
    console.append_child(&current_line).unwrap();
}

#[wasm_bindgen]
#[derive(Clone,Copy)]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("base-z47!");
}
