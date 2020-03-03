extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::clone;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent, PressEvent, Button, MouseButton, MouseCursorEvent};
use piston::window::WindowSettings;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    board : [u8;9],
    player : bool
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        

        let (w, h) = (args.window_size[0], args.window_size[1]);
        let (w13, w23) = (w / 3.0, 2.0 * w / 3.0);
        let (h13, h23) = (h / 3.0, 2.0 * h / 3.0);

        let pos : [[f64;2];9] = [
            [w /6.0, h /6.0 ],
            [3.0 * w /6.0, h /6.0 ],
            [5.0 * w /6.0, h /6.0 ],
            [w /6.0, 3.0 * h /6.0 ],
            [3.0 * w /6.0, 3.0 * h /6.0 ],
            [5.0 * w /6.0, 3.0 * h /6.0 ],
            [w /6.0, 5.0 * h /6.0 ],
            [3.0 * w /6.0, 5.0 * h /6.0 ],
            [5.0 * w /6.0, 5.0 * h /6.0 ]
        ];

        let board = self.board.clone();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);
            let transform = c
                .transform;

            line(BLACK,1.0,[w13,0.0,w13,h],transform,gl);
            line(BLACK,1.0,[w23,0.0,w23,h],transform,gl);
            line(BLACK,1.0,[0.0,h13,w,h13],transform,gl);
            line(BLACK,1.0,[0.0,h23,w,h23],transform,gl);

            for i in 0..board.len() {
                let value = board[i];
                let position = pos[i];
                if value == 1 {
                    let square = rectangle::square(position[0], position[1], 50.0);
                    rectangle(BLACK, square, c.transform.trans(-25.0,-25.0), gl);
                }
                else if value == 2 {
                    let circle = ellipse::circle(position[0], position[1], 25.0);
                    ellipse(BLACK, circle, c.transform , gl);
                }
            }
        });
    }

    fn clicked(&mut self, pos : &[f64; 2]) {
        if self.is_end() {
            self.restart();
        }
        else{
            let pos_number = self.get_pos(*pos);
            if self.board[pos_number] == 0 {
                match self.player {
                    true => self.board[pos_number] = 1,
                    false => self.board[pos_number] = 2
                }
            }
            self.player = !self.player
        }
    }

    fn is_end(&mut self) -> bool{
        let mut total = 0;
        for i in 0..self.board.len(){
            if self.board[i]>0 {
                total = total + 1
            }
        }
        if total > 8 {
            true
        }
        else{
            false
        }
    }

    fn restart(&mut self){
        self.board = [0,0,0,0,0,0,0,0,0]
    }
    
    fn get_pos(&self, pos : [f64;2]) -> usize {
        if pos[0]<66.6 {
            if pos[1] < 66.6 {
                0
            }
            else if pos[1] < 133.3 {
                3
            }
            else{
                6
            }
        }
        else if pos[0]< 133.3 {
            if pos[1]< 66.6 {
                1
            }
            else if pos[1]< 133.3 {
                4
            }
            else{
                7
            }
        }
        else{
            if pos[1] < 66.6 {
                2
            }
            else if pos[1] < 133.3 {
                5
            }
            else{
                8
            }
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;
    let mut cursor = [0.0, 0.0];

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("tic-tac-toe", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        board : [0,0,0,0,0,0,0,0,0],
        player : false
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        e.mouse_cursor(|pos| {
            cursor = pos;
        });
        if let Some(Button::Mouse(button)) = e.press_args() {
            app.clicked(&cursor);
        }
    }
}