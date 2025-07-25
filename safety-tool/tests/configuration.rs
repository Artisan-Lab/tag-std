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
    let toml: safety_tool::configuration::Configuration = toml::from_str(TOML).unwrap();
    dbg!(&toml);
}
