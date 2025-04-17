pub mod image0;
pub mod image1;
pub mod image2;
pub mod image3;
pub mod image4;

//use rgb::RGB8;
use hex::encode;

use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement, KeyboardEvent, HtmlCanvasElement, CanvasRenderingContext2d};



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


pub fn intro_image(ctx: &CanvasRenderingContext2d, width: u32, height: u32,image: usize) {
    for i in (0..8).rev() {
        draw_image_ex(ctx, width, height, image, i);
       // thread::sleep(Duration::from_millis(100));
    }
}

pub fn draw_image(ctx: &CanvasRenderingContext2d, width: u32, height: u32,image: usize) {
    draw_image_ex(ctx, width, height, image, 0);
}


#[wasm_bindgen]
pub fn draw_image_ex(ctx: &CanvasRenderingContext2d, width: u32, height: u32, image: usize, hide: u8) {

  
    
    let _sh = 60;
    let _sw = 200;
    
    let square_size = width / _sw;

    let pixels;

    if image == 0 {
        pixels = image0::get();
    } 
      else if image == 1 {
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

         //  let fc = encode(&pixels[j + 0..j + 3]);
         //      format!("#{:02X}", pixels[j + 0]);
        //        format!("{:02X}", pixels[j + 1]) +
        //        format!("{:02X}", pixels[j + 2]);

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


           //let bg = encode(&pixels[j + 0 + lsize..j + 3 + lsize]);
            //   format!("#{:02X}", pixels[j + 0 + lsize]) +
            //    format!("{:02X}", pixels[j + 1 + lsize]) +
            //    format!("{:02X}", pixels[j + 2 + lsize]);


            x = x + 1;
            count = count + 1;
        }
        x = 0;
        y =y + 2;
    }

}
