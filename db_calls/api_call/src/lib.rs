pub fn helloworld(name: &str) -> String {
    format!("welcome to Rust {}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helloworld() {
        let result = helloworld("Alice");
        assert_eq!(result, "welcome to Rust Alice");
    }

 
}
