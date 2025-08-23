use crate::primes::get_closest_prime;
use std::cmp::{max, min};

pub struct Matrix<T> {
    factor: usize,
    orig_size: usize,
    elements: Vec<(u32, u32, Option<T>)>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(size: usize) -> Matrix<T> {
        let prime = get_closest_prime(size as u32);
        Matrix {
            factor: prime,
            orig_size: size,
            elements: vec![(0, 0, None); prime * prime * 2],
        }
    }

    fn get_index(&self, id_x: u32, id_y: u32) -> usize {
        if id_x < id_y {
            (id_x as usize % self.factor) + self.orig_size * (id_y as usize % self.factor)
        } else {
            (id_y as usize % self.factor) + self.orig_size * (id_x as usize % self.factor)
        }
    }

    pub fn get_value(&self, id_x: u32, id_y: u32) -> Option<T> {
        let mut index = self.get_index(id_x, id_y);

        loop {
            if index >= self.elements.len() {
                return None;
            }
            if self.elements[index].0 == min(id_x, id_y)
                && self.elements[index].1 == max(id_x, id_y)
            {
                return Some(self.elements[index].2.clone().unwrap());
            }
            index += 1;
        }
    }

    pub fn set_value(&mut self, id_x: u32, id_y: u32, value: T) {
        let mut index = self.get_index(id_x, id_y);

        loop {
            if index >= self.elements.len() {
                return;
            }
            if (self.elements[index].0 == min(id_x, id_y)
                && self.elements[index].1 == max(id_x, id_y))
                || (self.elements[index].0 == 0 && self.elements[index].1 == 0)
            {
                self.elements[index] = (min(id_x, id_y), max(id_x, id_y), Some(value));
                return;
            }
            index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_set_value() {
        let mut matrix = Matrix::new(3);
        matrix.set_value(123, 345, 3);
        assert_eq!(matrix.get_value(123, 345), Some(3));
        assert_eq!(matrix.get_value(345, 123), Some(3));
        matrix.set_value(567, 345, 4);
        assert_eq!(matrix.get_value(567, 345), Some(4));
        assert_eq!(matrix.get_value(345, 567), Some(4));
    }
}
