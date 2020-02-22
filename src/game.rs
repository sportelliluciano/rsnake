extern crate rand;

use crate::graphics::Graphics;
use crate::events::Events;

use std::collections::VecDeque;
use rand::Rng;

const DIR_NONE: u32 = 0;
const DIR_LEFT: u32 = 1;
const DIR_RIGHT: u32 = 2;
const DIR_UP: u32 = 3;
const DIR_DOWN: u32 = 4;

const STEP_TIME: u32 = 250;

pub struct Game {
    snake: VecDeque<(i32, i32)>,
    map_size: (i32, i32),
    fruit_pos: (i32, i32),
    direction: u32,
    new_direction: u32,
    time_ms: u32,
    next_step_ms: u32,
    pub game_over: bool,
    pub is_win: bool,
}

impl Game {
    pub fn new(map_width: i32, map_height: i32) -> Game {
        let mut game = Game {
            snake: VecDeque::new(),
            map_size: (map_width, map_height),
            fruit_pos: (map_width-1, map_height-1),
            direction: DIR_NONE,
            new_direction: DIR_NONE,
            time_ms: 0,
            next_step_ms: 0,
            game_over: false,
            is_win: false
        };
        game.snake.push_back((0,0));
        game
    }

    pub fn step(&mut self, dt_ms: u32, events: &mut Events) {
        self.update_timer(dt_ms);
        self.read_new_direction(events);

        if self.game_over || self.next_step_ms > self.time_ms {
            return;
        }

        self.next_step_ms = self.time_ms + STEP_TIME;

        self.update_snake_direction();
        let new_head = self.calculate_new_head();
        if self.is_on_fruit(new_head) {
            // If the snake ate, we don't remove the tail so when we add the
            // new head it looks like it's growing.
            self.generate_new_fruit();
        } else {
            // If the snake didn't eat, we remove the last bit so it looks like
            // it moved.
            self.snake.pop_back();
        }

        // Check if the game is over and update game state
        self.update_game_state(new_head);

        if self.game_over {
            return;
        }

        // Add the new head to the snake.
        self.snake.push_front(new_head);
    }

    pub fn draw(&self, graphics: &mut Graphics) {
        graphics.create_empty_board(self.map_size);
        graphics.draw_fruit_cell(self.fruit_pos.0, self.fruit_pos.1);
        self.draw_snake(graphics);
        graphics.present_frame();
    }

    fn update_timer(&mut self, dt_ms: u32) {
        self.time_ms += dt_ms;
    }

    fn read_new_direction(&mut self, events: &mut Events) {
        if events.key_down && self.direction != DIR_UP {
            self.new_direction = DIR_DOWN;
        } else if events.key_up && self.direction != DIR_DOWN {
            self.new_direction = DIR_UP;
        } else if events.key_left && self.direction != DIR_RIGHT {
            self.new_direction = DIR_LEFT;
        } else if events.key_right && self.direction != DIR_LEFT {
            self.new_direction = DIR_RIGHT;
        }
    }

    fn update_snake_direction(&mut self) {
        if self.new_direction != DIR_NONE {
            self.direction = self.new_direction;
            self.new_direction = DIR_NONE;
        }
    }

    fn calculate_new_head(&self) -> (i32, i32) {
        let mut position = *self.snake.front().unwrap();
        match self.direction {
            DIR_RIGHT => position.0 += 1,
            DIR_DOWN  => position.1 += 1,
            DIR_LEFT  => position.0 -= 1,
            DIR_UP    => position.1 -= 1,
            _ => {}
        }
        return position;
    }

    fn is_on_fruit(&self, position: (i32, i32)) -> bool {
        position == self.fruit_pos
    }

    fn generate_new_fruit(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(0, self.map_size.0);
            let y = rng.gen_range(0, self.map_size.1);

            if !self.is_in_snake((x, y)) {
                self.fruit_pos = (x, y);
                break;
            }
        }
    }

    fn update_game_state(&mut self, new_head: (i32, i32)) {
        let user_won = self.user_has_won();
        if user_won || self.user_has_lost(new_head) {
            self.game_over = true;
            self.is_win = user_won;
        }
    }

    fn user_has_won(&self) -> bool {
        self.snake.len() >= ((self.map_size.0 * self.map_size.1) / 2) as usize
    }

    fn user_has_lost(&self, new_head: (i32, i32)) -> bool {
        self.is_out_of_map(new_head) || self.is_in_snake(new_head)
    }

    fn is_out_of_map(&self, position: (i32, i32)) -> bool {
        position.0 >= self.map_size.0 ||
        position.0 < 0 ||
        position.1 >= self.map_size.1 ||
        position.1 < 0
    }

    fn is_in_snake(&self, position: (i32, i32)) -> bool {
        for snake_segment in self.snake.iter() {
            if position == *snake_segment {
                return true;
            }
        }

        return false;
    }

    fn draw_snake(&self, graphics: &mut Graphics) {
        for snake_part in self.snake.iter() {
            graphics.draw_snake_cell(snake_part.0, snake_part.1);
        }
    }
}