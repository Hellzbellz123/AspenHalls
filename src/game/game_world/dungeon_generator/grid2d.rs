use bevy::math::Vec2;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid2D<T> {
    size: Vec2,
    data: Vec<T>,
}

impl<T> Grid2D<T> {
    fn new(size: Vec2, default_value: T) -> Self {
        let capacity = (size.x * size.y) as usize;
        let data = vec![default_value; capacity];

        Grid2D { size, data }
    }

    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x >= 0 && x < self.size.x as i32 && y >= 0 && y < self.size.y as i32 {
            let index = (x + y * self.size.x as i32) as usize;
            Some(&self.data[index])
        } else {
            None
        }
    }

    fn set(&mut self, x: i32, y: i32, value: T) {
        if x >= 0 && x < self.size.x as i32 && y >= 0 && y < self.size.y as i32 {
            let index = (x + y * self.size.x as i32) as usize;
            self.data[index] = value;
        }
    }
}

impl<T> Index<Vec2> for Grid2D<T> {
    type Output = T;

    fn index(&self, index: Vec2) -> &Self::Output {
        let index = (index.x + index.y * self.size.x) as usize;
        &self.data[index]
    }
}

impl<T> IndexMut<Vec2> for Grid2D<T> {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        let index = (index.x + index.y * self.size.x) as usize;
        &mut self.data[index]
    }
}
