use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

//Helper functions


// with constants \/ is mandatory
const BLOCK_SIZE: f64 = 25.0; //one block has 25 pixels

pub fn to_coord(game_coord: i32) -> f64{
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32{
    to_coord(game_coord) as u32
}

//function for drawing blocks on gui
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    //tuple like struct for rectangle - so we can change colors for snake, apple, and background
    rectangle(
        color,
        [
            gui_x, 
            gui_y, 
            BLOCK_SIZE, 
            BLOCK_SIZE
        ],
        con.transform,
        g,
    );
}

//similar to function above but this one shapes rectangles
pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(
        color,
        [
           x, 
           y, 
           BLOCK_SIZE * (width as f64),
           BLOCK_SIZE * (height as f64),  
        ],
        con.transform,
        g,
    );
}

