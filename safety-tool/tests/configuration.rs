use expect_test::expect;
use safety_tool::configuration::Configuration;

const TOML: &str = r#"
package.name = "core"
tool-version = "0.1.0"

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
    let s = &std::fs::read_to_string("assets/sp-core.toml").unwrap();
    let toml: Configuration = toml::from_str(s).unwrap();
    expect!["26"].assert_eq(&toml.tag.len().to_string());
}
