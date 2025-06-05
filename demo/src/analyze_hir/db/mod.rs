mod storage;
pub use storage::Database;

mod data;
pub use data::{Data, Func, PrimaryKey, Property, ToolAttrs};

pub fn get_all_tool_attrs(iter: impl IntoIterator<Item = Data>) -> crate::Result<ToolAttrs> {
    let mut db = Database::new("data.sqlite3")?;

    db.save_data(iter)?;
    let v_data = db.get_all_data()?;

    Ok(ToolAttrs::new(&v_data))
}
