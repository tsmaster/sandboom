#![allow(unused)]
use clap::Parser;
use macroquad::prelude::*;
use macroquad::input::*;
use macroquad::shapes::*;

//use macroquad_canvas_2d::*;
//use macroquad_canvas::*;

mod canvas;
use canvas::*;

use array2d::Array2D;

mod chunkframe;
mod sand;

use crate::chunkframe::ChunkFrame;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 768;

fn window_conf() -> Conf {
    Conf {
        window_title: "SandBoom".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}

fn reset(frame: &mut ChunkFrame) {
    for y in 0..60 {
        for x in 0..80 {
            frame.cells[(x, y)] = 0;
        }
    }
    for x in 0..80 {
        frame.cells[(x, 55)] = 1;
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    //let args = Cli::parse();
    println!("Hello, sandboom world!");

    let maximum_frame_rate = 120.0; // frames per second
    let minimum_frame_time = 1.0 / maximum_frame_rate;

    let canvas_width = 80.0;
    let canvas_height = 60.0;

    let mut frames = [
        ChunkFrame{
            cells: Array2D::filled_with(0, 100, 100),
        },
        ChunkFrame{
            cells: Array2D::filled_with(0, 100, 100),
        }
    ];

    reset(&mut frames[0]);
    reset(&mut frames[1]);
    
    let canvas = Canvas2D::new(canvas_width as f32, canvas_height as f32);

    clear_background(BLACK);

    canvas.get_texture().set_filter(macroquad::texture::FilterMode::Nearest);

    let mut this_frame_index = 0;
    let mut next_frame_index = 1;

    let mut desired_start_time = 0.0;
    
    loop {
        let loop_start_time = macroquad::time::get_time();

        if loop_start_time < desired_start_time {
            set_default_camera();
            canvas.draw();
            
            next_frame().await;
            continue;
        }
        desired_start_time = loop_start_time + minimum_frame_time;
        
        set_camera(&canvas.camera);
        //clear_background(WHITE);
        clear_background(BLACK);
        
        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            let (canvas_mouse_x, canvas_mouse_y) = canvas.mouse_position();
            
            //println!("{} {}", mouse_x, mouse_y);
            //draw_rectangle(canvas_mouse_x, canvas_mouse_y,
            //               2.0, 2.0, YELLOW);
            frames[this_frame_index].cells[(canvas_mouse_x as usize, canvas_mouse_y as usize)] = 3;
        }

        if is_key_down(KeyCode::R) {
            reset(&mut frames[this_frame_index]);
        }

        // clear next frame

        for y in 0..60 {
            for x in 0..80 {
                frames[next_frame_index].cells[(x,y)] = 0;
            }
        }        

        // tick

        for y in 0..60 {
            for x in 0..80 {
                let cell_val = frames[this_frame_index].cells[(x,y)];
                if cell_val == 0 {
                    continue;
                }

                if cell_val == 1 {
                    // wall
                    frames[next_frame_index].cells[(x,y)] = 1;
                }

                if cell_val == 3 {
                    // sand
                    sand::update_sand(&mut frames,
                                      this_frame_index,
                                      next_frame_index,
                                      x, y);
                }
            }
        }        

        

        // draw

        for y in 0..60 {
            for x in 0..80 {
                let c = frames[this_frame_index].cells[(x,y)];
                let mut paint_color = BLACK;
                if c == 1 {
                    paint_color = RED;
                } else if c == 2 {
                    paint_color = BLUE;
                } else if c == 3 {
                    paint_color = YELLOW;
                }
                draw_rectangle(x as f32, y as f32, 1.0, 1.0, paint_color);
            }
        }

        
        set_default_camera();

        canvas.draw();

        this_frame_index = (this_frame_index + 1) % 2;
        next_frame_index = (next_frame_index + 1) % 2;
        
        next_frame().await
    }
}
