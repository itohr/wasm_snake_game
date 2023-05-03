use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/client/utils/rnd.js")]
extern "C" {
    fn rnd(max: usize) -> usize;
}

#[wasm_bindgen]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(PartialEq, Clone, Copy)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body: Vec<SnakeCell> = vec![];

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        let size: usize = width * width;
        let snake: Snake = Snake::new(snake_idx, 3);

        World {
            width,
            size,
            reward_cell: World::gen_reward_cell(size, &snake.body),
            snake: snake,
            next_cell: None,
        }
    }

    fn gen_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell: usize;

        loop {
            reward_cell = rnd(max);
            if !(snake_body.contains(&SnakeCell(reward_cell))) {
                break;
            }
        }

        reward_cell
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction) {
        let next_cell: SnakeCell = self.get_next_cell(&direction);
        if next_cell.0 != self.snake.body[1].0 {
            self.snake.direction = direction;
            self.next_cell = Some(next_cell);
        } else {
            return;
        }
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn snake_body(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        let temp: Vec<SnakeCell> = self.snake.body.clone();

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.get_next_cell(&self.snake.direction);
            }
        }

        let len: usize = self.snake.body.len();

        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0);
        }

        if self.reward_cell == self.snake_head_idx() {
            self.snake.body.push(self.snake.body[1]);

            if self.snake.body.len() < self.size {
                self.reward_cell = World::gen_reward_cell(self.size, &self.snake.body);
            } else {
                self.reward_cell = self.size + 1;
            }
        }
    }

    fn get_next_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_idx: usize = self.snake_head_idx();
        let row: usize = snake_idx / self.width;
        return match direction {
            Direction::Right => SnakeCell((row * self.width) + (snake_idx + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_idx - 1) % self.width),
            Direction::Up => SnakeCell((snake_idx - self.width) % self.size),
            Direction::Down => SnakeCell((snake_idx + self.width) % self.size),
        };
    }
}
