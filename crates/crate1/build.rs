use std::path::PathBuf;

fn main() {
    let generated = r#"pub const HELLO: u16 = 308;"#;

    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    std::fs::write(out.join("generated"), generated).unwrap();
}
