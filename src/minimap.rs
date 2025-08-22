use crate::primes::get_closest_prime;

pub struct Minimap<T> {
    factor: usize,
    elements: Vec<(u32, Option<T>)>,
}

impl<T: Clone> Minimap<T> {
    pub fn new(size: usize) -> Minimap<T> {
        let prime = get_closest_prime(size as u32);
        Minimap {
            factor: prime,
            elements: vec![(0, None); prime * 2],
        }
    }

    fn get_index(&self, id: u32) -> usize {
        (id % self.factor as u32) as usize
    }

    pub fn get_value(&self, id: u32) -> Option<T> {
        let mut index = self.get_index(id);

        loop {
            if index >= self.elements.len() {
                return None;
            }
            if self.elements[index].0 == id {
                return Some(self.elements[index].1.clone().unwrap());
            }
            index += 1;
        }
    }

    pub fn set_value(&mut self, id: u32, value: T) {
        let mut index = self.get_index(id);
        loop {
            if index >= self.elements.len() {
                return;
            }
            if self.elements[index].0 == 0 || self.elements[index].0 == id {
                self.elements[index] = (id, Some(value));
                return;
            }
            index += 1;
        }
    }

    pub fn all(&self, cb: fn(v: &T) -> bool) -> bool {
        self.elements.iter().all(|x| {
            if x.1.is_none() {
                return true;
            } else {
                cb(&x.1.clone().unwrap())
            }
        })
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
        let mut minimap = Minimap::new(3);
        minimap.fill_with(&vec![(123, 2), (567, 2)]);
        assert_eq!(minimap.get_value(123), Some(2));
        assert_eq!(minimap.get_value(567), Some(2));
    }

    #[test]
    fn test_set_value() {
        let mut minimap = Minimap::new(3);
        minimap.set_value(123, 3);
        assert_eq!(minimap.get_value(123), Some(3));
        minimap.set_value(567, 4);
        assert_eq!(minimap.get_value(567), Some(4));
    }
}
