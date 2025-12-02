pub mod print {

    pub fn world() ->String {
        String::from("Hello, World!")
    }
}


#[cfg(test)]
mod tests {
    use crate::print::world;

    #[test]
    fn world_returns_hello_world() {
        assert_eq!(world(), "Hello, World!", "Expecting 'Hello, World!', found '{}'", world());
    }
}