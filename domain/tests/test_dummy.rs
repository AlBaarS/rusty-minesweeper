#[cfg(test)]
mod tests {
    use crate::domain::dummy;

    #[test]
    fn assert_dummy_is_true() -> () {
        assert_eq!(dummy(), true)
    }
}