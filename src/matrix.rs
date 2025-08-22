pub struct Matrix<T> {
    elements: Vec<T>,
    id_to_index_map: Vec<u32>,
}

impl<T: Clone> Matrix<T> {
    pub fn new(ids: &Vec<u32>, fill: T) -> Matrix<T> {
        let mut map = ids.clone();
        // need to sort to use binary search
        map.sort_by(|a, b| a.cmp(b));
        Matrix {
            elements: vec![fill; ids.len() * ids.len()],
            id_to_index_map: map,
        }
    }

    fn get_index(&self, id: u32) -> Option<usize> {
        match self.id_to_index_map.binary_search(&id) {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }

    pub fn get_value(&self, id_x: u32, id_y: u32) -> Option<&T> {
        let index_x = self.get_index(id_x);
        let index_y = self.get_index(id_y);
        if let (Some(mut i_x), Some(mut i_y)) = (index_x, index_y) {
            if i_x > i_y {
                std::mem::swap(&mut i_x, &mut i_y);
            }
            Some(&self.elements[i_x + self.id_to_index_map.len() * i_y])
        } else {
            None
        }
    }

    pub fn set_value(&mut self, id_x: u32, id_y: u32, value: T) {
        let index_x = self.get_index(id_x);
        let index_y = self.get_index(id_y);
        if let (Some(mut i_x), Some(mut i_y)) = (index_x, index_y) {
            if i_x > i_y {
                std::mem::swap(&mut i_x, &mut i_y);
            }
            self.elements[i_x + self.id_to_index_map.len() * i_y] = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;

    #[test]
    fn test_create_matrix() {
        let matrix = Matrix::new(&vec![567, 345, 123], 2);
        assert_eq!(matrix.get_value(123, 345), Some(&2));
        assert_eq!(matrix.get_value(123, 567), Some(&2));
    }

    #[test]
    fn test_set_value() {
        let mut matrix = Matrix::new(&vec![567, 345, 123], 2);
        matrix.set_value(123, 345, 3);
        assert_eq!(matrix.get_value(123, 345), Some(&3));
        assert_eq!(matrix.get_value(345, 123), Some(&3));
        matrix.set_value(567, 345, 4);
        assert_eq!(matrix.get_value(567, 345), Some(&4));
        assert_eq!(matrix.get_value(345, 567), Some(&4));
    }
}
