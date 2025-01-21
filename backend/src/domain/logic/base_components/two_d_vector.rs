use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
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
        return self.matrix[y][x].clone();
    }

    pub fn change_element(&self, x: usize, y: usize, new_element: T) -> TwoDVector<T> {
        let mut changed_vector: Vec<Vec<T>> = self.get_vec();
        changed_vector[y][x] = new_element;
        TwoDVector {
            matrix: changed_vector,
            size: self.get_size(),
        }
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
        assert_eq!(vector.get_element(0, 1), 3);
    }

    #[test]
    fn test_if_specific_element_is_changed() -> () {
        let vector_input: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
        let vector_output: Vec<u8> = vec![1,2,5,4,5,6,7,8,9];
        let two_d_vector_in: TwoDVector<u8> = TwoDVector::new(vector_input, 3);
        let two_d_vector_out: TwoDVector<u8> = TwoDVector::new(vector_output, 3);
        assert_eq!(two_d_vector_in.change_element(2, 0, 5).get_vec(), two_d_vector_out.get_vec());
    }

    #[test]
    fn test_if_boolean_flipping_from_false_to_true_is_possible() -> () {
        let vector_input: Vec<bool> = vec![false, false, false, false, false, false, false, false, false];
        let vector_output: Vec<bool> = vec![false, false, false, true, false, false, false, false, false];
        let two_d_vector_in: TwoDVector<bool> = TwoDVector::new(vector_input, 3);
        let two_d_vector_out: TwoDVector<bool> = TwoDVector::new(vector_output, 3);
        let two_d_vector_flipped: TwoDVector<bool> = two_d_vector_in.change_element(0, 1, !two_d_vector_in.get_element(0, 1));
        assert_eq!(two_d_vector_flipped.get_vec(), two_d_vector_out.get_vec());
    }

    #[test]
    fn test_if_boolean_flipping_from_true_to_false_is_possible() -> () {
        let vector_input: Vec<bool> = vec![false, false, false, true, false, false, false, false, false];
        let vector_output: Vec<bool> = vec![false, false, false, false, false, false, false, false, false];
        let two_d_vector_in: TwoDVector<bool> = TwoDVector::new(vector_input, 3);
        let two_d_vector_out: TwoDVector<bool> = TwoDVector::new(vector_output, 3);
        let two_d_vector_flipped: TwoDVector<bool> = two_d_vector_in.change_element(0, 1, !two_d_vector_in.get_element(0, 1));
        assert_eq!(two_d_vector_flipped.get_vec(), two_d_vector_out.get_vec());
    }
}
