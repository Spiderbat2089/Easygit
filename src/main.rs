use macroquad::prelude::*;
use macroquad::ui::root_ui;
#[macroquad::main("Easy Git")]
async fn main() {
    loop {
        clear_background(RED);

        root_ui().label(None, "hello megaui");
        if root_ui().button(vec2(90.0,80.0), "Push me") {
            println!("pushed");
        }
        

        next_frame().await;
    }
}