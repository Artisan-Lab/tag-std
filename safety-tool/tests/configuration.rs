use expect_test::expect;
use safety_parser::configuration::Configuration;

const TOML: &str = r#"
[tag.A]

[tag.ValidPtr]
args = [ "p", "T", "len" ]
desc = "A valid pointer."
expr = "Size(T, 0) || (!Size(T,0) && Deref(p, T, len))"
url = "https://doc.rust-lang.org/std/ptr/index.html#safety""#;

#[test]
fn deserialize() {
    let toml: Configuration = toml::from_str(TOML).unwrap();
    dbg!(&toml);
}

#[test]
fn core() {
    let config = &Configuration::read_toml("assets/sp-core.toml");
    expect!["26"].assert_eq(&config.tag.len().to_string());
}

#[test]
fn rust_for_linux() {
    let config = &Configuration::read_toml("assets/sp-rust-for-linux.toml");
    expect!["39"].assert_eq(&config.tag.len().to_string());
}
