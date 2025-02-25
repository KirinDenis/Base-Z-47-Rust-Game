mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement, KeyboardEvent};

#[wasm_bindgen(start)]
pub fn start() {
    let document = window().unwrap().document().unwrap();


    let console = document.create_element("div").unwrap();
    console.set_attribute("id", "console").unwrap();
    console.set_attribute("tabindex", "0").unwrap(); 
    document.body().unwrap().append_child(&console).unwrap();


    let line = document.create_element("div").unwrap();
    line.set_attribute("class", "line").unwrap();

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

    console.append_child(&line).unwrap();


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
