#[cfg(test)]
mod tests {
    use rusty_minesweeper::domain::logic::dummy;

    #[test]
    fn assert_dummy_is_true() -> () {
        assert_eq!(dummy::dummy(), true)
    }
}

fn main() {
    println!("Hello, world!");
}