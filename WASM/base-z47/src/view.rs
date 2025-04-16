pub mod image0;
//pub mod image1;
//pub mod image2;
//pub mod image3;
//pub mod image4;

//use rgb::RGB8;
use hex::encode;

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement, KeyboardEvent};



pub fn intro_image(image: usize) {
    for i in (0..8).rev() {
        draw_image_ex(image, i);
       // thread::sleep(Duration::from_millis(100));
    }
}

pub fn draw_image(image: usize) {
    draw_image_ex(image, 0);
}

#[wasm_bindgen]
pub fn draw_image_ex(image: usize, hide: u8) {

    let mut buffer: Vec<String> = Vec::new();

//    let (_sh, _sw) = term.size();
    
    let _sh = 60;
    let _sw = 200;
    

    let pixels;

    if image == 0 {
        pixels = image0::get();
    } 
      else if image == 1 {
  pixels = image0::get();
    } else if image == 2 {
  pixels = image0::get();
    } else if image == 3 {
  pixels = image0::get();
    } else {
  pixels = image0::get();
    }

  //  term.move_cursor_to(0, 0).unwrap();

    let psize = 56000;
    let lsize = 200 * 3;
//    let psize = 560;
//    let lsize = 20 * 3;

    let mut count: usize = 0;

    for i in (0..psize).step_by(lsize * 2) {
        let mut s1: String = Default::default();

        for j in (i..i + lsize).step_by(3) {

           let fc = encode(&pixels[j + 0..j + 3]);
         //      format!("#{:02X}", pixels[j + 0]);
        //        format!("{:02X}", pixels[j + 1]) +
        //        format!("{:02X}", pixels[j + 2]);

           let bg = encode(&pixels[j + 0 + lsize..j + 3 + lsize]);
            //   format!("#{:02X}", pixels[j + 0 + lsize]) +
            //    format!("{:02X}", pixels[j + 1 + lsize]) +
            //    format!("{:02X}", pixels[j + 2 + lsize]);

              
/*

use hex::encode;

fn main() {
    let data: Vec<u8> = (0..=255).collect();
    let hex_string = encode(&data[4..7]); 
    println!("{}", hex_string); // "040506"
}

            let fc: RGB8 = RGB8 {
                r: pixels[j + 0] >> hide,
                g: pixels[j + 1] >> hide,
                b: pixels[j + 2] >> hide,
            };

fn main() {
    let byte: u8 = 255;
    let hex_string = format!("{:02X}", byte); // "FF"
    println!("{}", hex_string);
}

            let bc: RGB8 = RGB8 {
                r: pixels[j + 0 + lsize] >> hide,
                g: pixels[j + 1 + lsize] >> hide,
                b: pixels[j + 2 + lsize] >> hide,
            };
*/

//            let style: Style = Style::new()
//                .fg(Color::Color256(ansi256_from_rgb(fc)))
//                .bg(Color::Color256(ansi256_from_rgb(bc)));

            //s1.push_str(&style.apply_to("\u{2580}").to_string());
            s1.push_str(&format!("<span style='color: #{}; background-color: #{};'>{}</span>", fc, bg, "\u{2580}"));
            count = count + 1;
        }
        count = count + 2;
        buffer.push(s1);
        buffer.push("<br>".to_string());
    }

    let document = window().unwrap().document().unwrap();
    let code = document.get_element_by_id("code");
    code.expect("REASON").set_inner_html(&buffer.join(" "));
       

//    term.move_cursor_to(0, 0).unwrap();
//    for text in buffer {
//        term.write_line(&text).unwrap();
//    }


}
