use crate::Result;

pub fn run(args: &[String]) -> Result<()> {
    let vars = vec![("RUSTC_BOOTSTRAP", "1")];

    safety_tool::utils_cmd::execute("safety-tool", args, vars)?;

    Ok(())
}
