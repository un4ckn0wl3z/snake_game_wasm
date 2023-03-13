#![allow(dead_code)]

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC:  WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}


#[wasm_bindgen]
pub fn greeting(name: &str) {
    alert(name);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[derive(Clone)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = vec!();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i))
        }

        Snake { body, direction: Direction::Right }
    }
}

#[wasm_bindgen]
pub struct World {
     width: usize,
     size: usize,
     snake: Snake
}

#[wasm_bindgen]
impl World {
    pub fn new (width: usize, snake_idx: usize) -> Self {
        Self { width, size: width * width, snake: Snake::new(snake_idx, 3) }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn change_snake_dir(&mut self, direction: Direction){
        self.snake.direction = direction
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    // *const is raw pointer
    // borrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell{
        self.snake.body.as_ptr()
    }
 
    // cannot return reference to JS because of borrowing rules
    // pub fn snake_cells(&self) -> &Vec<SnakeCell>{
    //     &self.snake.body
    // }


    pub fn step(&mut self) {
        let temp = self.snake.body.clone();
        let next_cell = self.gen_next_snake_cell();
        self.snake.body[0] = next_cell;

        let len = self.snake.body.len();
        for i in 1..len {
            self.snake.body[i] = SnakeCell(temp[i - 1].0)
        }

    }

    fn gen_next_snake_cell(&self) -> SnakeCell {
        let snake_idx = self.snake_head_idx();
        let row = snake_idx / self.width;

        return match self.snake.direction {
            Direction::Right => {
                let treshold = (row + 1) * self.width;
                if snake_idx + 1 == treshold {
                    SnakeCell(treshold - self.width)
                } else {
                    SnakeCell(snake_idx + 1)
                }
            },
            Direction::Left => {
                let treshold = row * self.width;
                if snake_idx == treshold {
                    SnakeCell(treshold + (self.width - 1))
                } else {
                    SnakeCell(snake_idx - 1)
                }
            },
            Direction::Up => {
                let treshold = snake_idx - (row * self.width);
                if snake_idx == treshold {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_idx - self.width)
                }
            },
            Direction::Down => {
                let treshold = snake_idx + ((self.width - row) * self.width);
                if snake_idx + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_idx + self.width)
                }
            },
        };

    }

 
}