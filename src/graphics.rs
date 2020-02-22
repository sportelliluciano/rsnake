extern crate sdl2;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::VideoSubsystem;

const WINDOW_WIDTH: i32 = 400;
const WINDOW_HEIGHT: i32 = 400;

const BOARD_PADDING_PX: i32 = 9;
const CELL_PADDING_PX: i32 = 2;

const FRUIT_COLOR: Color = Color::RGB(0, 0, 100);
const SNAKE_COLOR: Color = Color::RGB(100, 0, 0);
const BOARD_COLOR: Color = Color::RGB(0, 100, 0);

pub struct Graphics {
    canvas: Canvas<Window>,
    margin: i32,
    cell_size: i32,
}

impl Graphics {
    pub fn new(video: VideoSubsystem) -> Graphics {
        let window = video
            .window("RSnake", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap();

        Graphics {
            canvas: canvas,
            cell_size: 0,
            margin: 0
        }
    }

    pub fn create_empty_board(&mut self, map_size: (i32, i32)) {
        let (map_width, map_height) = map_size;

        let cell_size_with_padding = (WINDOW_WIDTH - (2 * BOARD_PADDING_PX)) / map_width;
        self.cell_size = cell_size_with_padding - 2 * CELL_PADDING_PX;
        
        let width_px = map_width * cell_size_with_padding;
        let height_px = map_height * cell_size_with_padding;
        self.margin = (WINDOW_WIDTH - width_px) / 2;
        
        self.canvas.set_draw_color(BOARD_COLOR);
        self.canvas.fill_rect(sdl2::rect::Rect::new(self.margin, 
            self.margin, width_px as u32, height_px as u32)).unwrap();
    }

    pub fn draw_snake_cell(&mut self, x: i32, y: i32) {
        self.canvas.set_draw_color(SNAKE_COLOR);
        let (x_px, y_px) = self.cell_to_px(x, y);
        self.canvas.fill_rect(sdl2::rect::Rect::new(x_px, y_px, 
            self.cell_size as u32, self.cell_size as u32)).unwrap();
    }

    pub fn draw_fruit_cell(&mut self, x: i32, y: i32) {
        self.canvas.set_draw_color(FRUIT_COLOR);
        let (x_px, y_px) = self.cell_to_px(x, y);
        self.canvas.fill_rect(sdl2::rect::Rect::new(x_px, y_px, 
            self.cell_size as u32, self.cell_size as u32)).unwrap();
    }

    pub fn present_frame(&mut self) {
        self.canvas.present();
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
    }

    fn cell_to_px(&self, x: i32, y: i32) -> (i32, i32) {
        let x_px = self.margin + CELL_PADDING_PX + x * (self.cell_size + 2 * CELL_PADDING_PX);
        let y_px = self.margin + CELL_PADDING_PX + y * (self.cell_size + 2 * CELL_PADDING_PX);
        return (x_px, y_px);
    }
}