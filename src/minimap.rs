pub struct Minimap<T> {
    elements: Vec<T>,
    id_to_index_map: Vec<u32>,
}

impl<T: Clone> Minimap<T> {
    pub fn new(ids: &Vec<u32>, fill: T) -> Minimap<T> {
        let mut map = ids.clone();
        // need to sort to use binary search
        map.sort_by(|a, b| a.cmp(b));
        Minimap {
            elements: vec![fill; ids.len()],
            id_to_index_map: map,
        }
    }

    fn get_index(&self, id: u32) -> Option<usize> {
        match self.id_to_index_map.binary_search(&id) {
            Ok(i) => Some(i),
            Err(_) => None,
        }
    }

    pub fn get_value(&self, id: u32) -> Option<&T> {
        let index = self.get_index(id);
        if let Some(i) = index {
            Some(&self.elements[i])
        } else {
            None
        }
    }

    pub fn set_value(&mut self, id: u32, value: T) {
        let index = self.get_index(id);
        if let Some(i) = index {
            self.elements[i] = value;
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.elements.iter()
    }

    pub fn fill_with(&mut self, value: &Vec<(u32, T)>) {
        value.iter().for_each(|(id, value)| {
            self.set_value(*id, value.clone());
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::minimap::Minimap;

    #[test]
    fn test_create_minimap() {
        let matrix = Minimap::new(&vec![567, 345, 123], 2);
        assert_eq!(matrix.get_value(123), Some(&2));
        assert_eq!(matrix.get_value(123), Some(&2));
    }

    #[test]
    fn test_set_value() {
        let mut matrix = Minimap::new(&vec![567, 345, 123], 2);
        matrix.set_value(123, 3);
        assert_eq!(matrix.get_value(123), Some(&3));
        matrix.set_value(567, 4);
        assert_eq!(matrix.get_value(567), Some(&4));
    }
}
