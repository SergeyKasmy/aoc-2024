use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::Point;

#[derive(Clone, Copy)]
pub struct Grid<T, const N: usize>(pub [[T; N]; N]);

impl<T, const N: usize> Grid<T, N> {
    pub fn get(&self, point: Point<usize>) -> Option<&T> {
        if point.x >= N || point.y >= N {
            return None;
        }

        Some(&self[point])
    }
}

impl<T, const N: usize> Default for Grid<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self([[T::default(); N]; N])
    }
}

impl<T, const N: usize> Index<Point<usize>> for Grid<T, N> {
    type Output = T;

    fn index(&self, index: Point<usize>) -> &Self::Output {
        &self.0[index.x][index.y]
    }
}

impl<T, const N: usize> IndexMut<Point<usize>> for Grid<T, N> {
    fn index_mut(&mut self, index: Point<usize>) -> &mut Self::Output {
        &mut self.0[index.x][index.y]
    }
}

impl<T, const N: usize> Deref for Grid<T, N> {
    type Target = [[T; N]; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Grid<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
