pub struct TwoDVector<T: Clone> {
    matrix: Vec<Vec<T>>,
}

impl<T: Clone> TwoDVector<T> {
    pub fn new(vector: Vec<T>, size: usize) -> Self {
        let matrix: Vec<Vec<T>> = vector.chunks(size).map(|s| s.into()).collect();
        Self {
            matrix,
        }
    }

    pub fn get_vec(&self) -> Vec<Vec<T>> {
        self.matrix.clone()
    }
}

fn main() {}
