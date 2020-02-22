extern crate sdl2;

mod events;
mod graphics;
mod game;

use events::Events;
use graphics::Graphics;
use game::Game;

const FRAME_RATE_MS: u32 = 25;
const FIELD_SIZE: i32 = 15;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut timer = sdl.timer().unwrap();
    let video = sdl.video().unwrap();

    let mut events = Events::new(sdl.event_pump().unwrap());
    let mut graphics = Graphics::new(video);
    let mut game = Game::new(FIELD_SIZE, FIELD_SIZE);

    while !game.game_over && !events.quit && !events.key_escape {
        let t1 = timer.ticks();
        while timer.ticks() - t1 < FRAME_RATE_MS {
            timer.delay(1); // Release CPU to reduce load
        }
        
        game.draw(&mut graphics);
        game.step(timer.ticks() - t1, &mut events);
        
        events.pump();
    }

    if game.is_win {
        println!("You win!");
    } else {
        println!("You lose :(");
    }
}
