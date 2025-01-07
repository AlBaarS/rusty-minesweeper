#[cfg(test)]
mod tests {
    use rusty_minesweeper::domain::logic::two_d_vector::TwoDVector;

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
}

fn main() {}
