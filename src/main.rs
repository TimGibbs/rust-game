//! Shows how to render simple primitive shapes with a single color.
use macroquad::prelude::*;
#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        let wid = (screen_width()/HD/2.0 + 1.0) as i32;
        let hei = (screen_height()/VD +1.0) as i32;
        

        clear_background(LIGHTGRAY);
        draw_grid(hei,wid);

        next_frame().await
    }
}

fn draw_hex(x:f32, y:f32) {
    draw_hexagon(x, y, 50.0, 2.0, true, macroquad::color::LIME, GRAY);
}

fn draw_grid (height:i32, length:i32){
    let mut i = 0;
    while i<height+1 {
        draw_line(i, length);
        i+=1
    }
}

fn draw_line (y:i32, length:i32){
    let mut i = 0;
    let offset = if y%2 == 0 {
        0
    } else {
        1
    }; 
    while i<length +1 {
        draw_hex(hd(2*i+offset), vd(y));
        i+=1;
    }
}

const HD : f32 = 43.3;
const VD : f32 = 75.0;

fn hd(x:i32) -> f32 { (x as f32) * HD }
fn vd(y:i32) -> f32 { (y as f32) * VD }