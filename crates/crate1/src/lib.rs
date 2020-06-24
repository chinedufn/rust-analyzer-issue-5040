include!(concat!(env!("OUT_DIR"), "/generated.rs"));

pub const NOT_GENERATED: u16 = 524;

#[cfg(test)]
mod tests {
    use super::*;

    /// Verify that
    #[test]
    fn foo() {
        println!("HELLO constant has the value: {}", HELLO);
    }
}
