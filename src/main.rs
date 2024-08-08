use cli::say_hi;

fn main() {
    say_hi()
}

mod tests {
    #[test]
    fn it_works() {
        let result = cli::add(2, 2);
        assert_eq!(result, 4);
    }
}
