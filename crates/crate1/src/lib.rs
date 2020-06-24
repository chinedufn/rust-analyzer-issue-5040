include!(concat!(env!("OUT_DIR"), "/generated"));

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that
    #[test]
    fn foo() {
        println!("HELLO constant has the value: {}", HELLO);
    }
}
