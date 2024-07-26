fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn run_test() {
        assert_eq!(1, 1);
    }
}
