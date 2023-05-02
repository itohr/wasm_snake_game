use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec![SnakeCell(spawn_index)],
            direction: Direction::Down,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> World {
        World {
            width,
            snake: Snake::new(snake_idx),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get_snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn update(&mut self) {
        let snake_idx: usize = self.get_snake_head_idx();
        let (mut col, mut row): (usize, usize) = self.idx_to_cell(snake_idx);
        (col, row) = match self.snake.direction {
            Direction::Right => ((col + 1) % self.width, row),
            Direction::Left => ((col - 1) % self.width, row),
            Direction::Up => (col, (row - 1) % self.width),
            Direction::Down => (col, (row + 1) % self.width),
        };
        self.set_snake_head_idx(self.cell_to_idx(col, row));
    }

    fn set_snake_head_idx(&mut self, idx: usize) {
        self.snake.body[0].0 = idx;
    }

    fn idx_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }

    fn cell_to_idx(&self, col: usize, row: usize) -> usize {
        col + (row * self.width)
    }
}
