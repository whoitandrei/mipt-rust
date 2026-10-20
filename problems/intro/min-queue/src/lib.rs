#![forbid(unsafe_code)]

use std::collections::VecDeque;

#[derive(Default)]
pub struct MinQueue<T> {
    main_q: VecDeque<T>,
    min_q: VecDeque<T>,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        MinQueue { main_q: VecDeque::<T>::new(), min_q: VecDeque::<T>::new() }
    }

    pub fn push(&mut self, val: T) {
        self.main_q.push_back(val.clone());
        while self.min_q.back().is_some_and(|last| last > &val) {
            self.min_q.pop_back();
        }
        self.min_q.push_back(val);
    }

    pub fn pop(&mut self) -> Option<T> {
        let front = self.main_q.pop_front()?;

        if self.min_q.front().is_some_and(|min| min == &front) {
            self.min_q.pop_front();
        }

        Some(front)
    }

    pub fn front(&self) -> Option<&T> {
        self.main_q.front()
    }

    pub fn min(&self) -> Option<&T> {
        self.min_q.front()
    }

    pub fn len(&self) -> usize {
        self.main_q.len()
    }

    pub fn is_empty(&self) -> bool {
        self.main_q.is_empty()
    }
}
