extern crate sdl2;

mod events;
mod game;
mod graphics;

use events::Events;
use game::Game;
use graphics::Graphics;

const FRAME_RATE_MS: u32 = 25;
const FIELD_SIZE: i32 = 15;

fn main() {
    let sdl = sdl2::init().expect("Could not initialize SDL2.");
    let mut timer = sdl
        .timer()
        .expect("Could not initialize SDL2 Timers subsystem.");
    let video = sdl
        .video()
        .expect("Could not initialize SDL2 Video subsystem.");

    let mut events = Events::new(
        sdl.event_pump()
            .expect("Could not create an SDL2 events pump."),
    );
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
