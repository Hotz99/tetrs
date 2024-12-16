use crate::game;

use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct NextShapes {
    all_shapes: Vec<char>,
    available_shapes: Vec<char>,
    next_up_shapes: VecDeque<char>,
    stack_size: usize,
}

// TODO refactor this bs
impl NextShapes {
    pub fn new(lookahead_size: u8) -> NextShapes {
        let mut next_shapes = NextShapes {
            all_shapes: vec!['X', 'V', 'Z', 'W', 'I', 'T', 'Y', 'L', 'N', 'P', 'U', 'F'],
            available_shapes: Vec::new(),
            next_up_shapes: VecDeque::new(),
            stack_size: lookahead_size as usize,
        };

        next_shapes.generate_next_up_shapes();
        next_shapes
    }

    fn refresh(&mut self) {
        self.available_shapes.clear();
        self.available_shapes.extend(&self.all_shapes);
        let mut rng = thread_rng();
        self.available_shapes.shuffle(&mut rng);
    }

    fn generate_next_up_shapes(&mut self) {
        self.next_up_shapes.clear();
        for _ in 0..self.stack_size {
            let next = self.get_next_shape_from_pool();
            self.next_up_shapes.push_back(next);
        }
    }

    fn get_next_shape_from_pool(&mut self) -> char {
        if self.available_shapes.is_empty() {
            self.refresh();
        }
        self.available_shapes.remove(0)
    }

    pub fn get_next_stack(&mut self) -> Vec<char> {
        let mut next_stack = Vec::with_capacity(self.stack_size);

        for i in 0..self.stack_size {
            next_stack.push(self.next_up_shapes[i]);
        }

        self.next_up_shapes.pop_front();
        let next = self.get_next_shape_from_pool();
        self.next_up_shapes.push_back(next);

        next_stack
    }
}
