mod storage;
pub use storage::Database;

mod data;
pub use data::{Data, Func, PrimaryKey, Property, TagState, ToolAttrs, tool_attr_on_hir};

pub fn get_all_tool_attrs(iter: impl IntoIterator<Item = Data>) -> crate::Result<ToolAttrs> {
    // Recommend setting the DATA_SQLITE3 environment variable to an absolute path.
    // The default path is relative to the folder where the crate is being compiled,
    // leading to a sqlite3 file in each crate folder.
    const DATA_SQLITE3: &str = "data.sqlite3";
    let path = std::env::var("DATA_SQLITE3");
    let path = path.as_deref().unwrap_or(DATA_SQLITE3);

    let mut db = Database::new(path)?;

    db.save_data(iter)?;
    let v_data = db.get_all_data()?;

    Ok(ToolAttrs::new(&v_data))
}
