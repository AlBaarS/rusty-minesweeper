use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct TwoDVector<T: Clone> {
    matrix: Vec<Vec<T>>,
    size: u8,
}

impl<T: Clone> TwoDVector<T> {
    pub fn new(vector: Vec<T>, size: u8) -> Self {
        let matrix: Vec<Vec<T>> = vector.chunks(size.into()).map(|s| s.into()).collect();
        return Self {
            matrix,
            size,
        };
    }

    pub fn get_vec(&self) -> Vec<Vec<T>> {
        return self.matrix.clone();
    }

    pub fn get_size(&self) -> u8 {
        return self.size.clone();
    }

    pub fn get_element(&self, x: usize, y: usize) -> T {
        return self.get_vec()[x][y].clone();
    }
}


#[allow(dead_code)]
fn main() {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_2d_vector_from_numeric_vector() -> () {
        let vector_numeric: Vec<u8> = vec![1,2,3,4];
        assert_eq!(TwoDVector::new(vector_numeric, 2).get_vec(), vec![vec![1, 2], vec![3, 4]])
    }

    #[test]
    fn create_2d_vector_from_string_vector() -> () {
        let vector_strings: Vec<String> = vec!["Frodo".to_string(), "Sam".to_string(), "Merry".to_string(), "Pippin".to_string()];
        assert_eq!(TwoDVector::new(vector_strings, 2).get_vec(), 
        vec![
            vec!["Frodo".to_string(), "Sam".to_string()], 
            vec!["Merry".to_string(), "Pippin".to_string()]
        ])
    }

    #[test]
    fn get_specific_element_from_2d_vector() -> () {
        let vector_numeric: Vec<u8> = vec![1,2,3,4];
        let vector: TwoDVector<u8> = TwoDVector::new(vector_numeric, 2);
        assert_eq!(vector.get_element(1, 0), 3);
    }
}
