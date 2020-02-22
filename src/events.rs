extern crate sdl2;

use sdl2::EventPump;

pub struct Events {
    pump: EventPump,
    pub quit: bool,
    pub key_escape: bool,
    pub key_left: bool,
    pub key_right: bool,
    pub key_up: bool,
    pub key_down: bool
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
            key_left: false,
            key_right: false,
            key_up: false,
            key_down: false,
        }
    }

    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit {..} => self.quit = true,
                KeyDown { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = true,
                    Some(Left) => self.key_left = true,
                    Some(Right) => self.key_right = true,
                    Some(Up) => self.key_up = true,
                    Some(Down) => self.key_down = true,
                    _ => {}
                }
                KeyUp { keycode, .. } => match keycode {
                    Some(Escape) => self.key_escape = false,
                    Some(Left) => self.key_left = false,
                    Some(Right) => self.key_right = false,
                    Some(Up) => self.key_up = false,
                    Some(Down) => self.key_down = false,
                    _ => {}
                }

                _ => {}
            }
        }
    }
}