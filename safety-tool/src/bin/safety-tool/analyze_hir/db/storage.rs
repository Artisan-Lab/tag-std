use super::{Data, Func, PrimaryKey};
use crate::Result;
use eyre::Context;
use rusqlite::{Connection, named_params};

pub struct Database {
    conn: Connection,
}

const CREATE: &str = "
CREATE TABLE IF NOT EXISTS hir (
  hash1 INTEGER NOT NULL,
  hash2 INTEGER NOT NULL,
  tool_attrs TEXT NOT NULL,
  def_path TEXT NOT NULL,
  function TEXT NOT NULL,
  timestamp TEXT NOT NULL,
  PRIMARY KEY (hash1, hash2)
) STRICT;
";

impl Database {
    pub fn new(path: &str) -> Result<Database> {
        let conn = Connection::open(path).with_context(|| format!("Failed to open {path}"))?;
        conn.execute(CREATE, ()).with_context(|| format!("Failed to execute sql:\n{CREATE}"))?;
        Ok(Database { conn })
    }

    pub fn save_data(&mut self, iter: impl IntoIterator<Item = Data>) -> Result<()> {
        const UPSERT: &str = "
INSERT OR REPLACE INTO hir (hash1, hash2, tool_attrs, def_path, function, timestamp)
VALUES (:hash1, :hash2, :tool_attrs, :def_path, :function, :timestamp)
";
        let mut stmt = self.conn.prepare(UPSERT)?;

        let timestamp = jiff::Timestamp::now();
        for data in iter {
            let tool_attrs = serde_json::to_string_pretty(data.func.tool_attrs.as_slice())?;
            stmt.execute(named_params! {
                ":hash1": data.hash.hash1.cast_signed(),
                ":hash2": data.hash.hash2.cast_signed(),
                ":tool_attrs": tool_attrs,
                ":def_path": data.func.def_path,
                ":function": data.func.function,
                ":timestamp": timestamp,
            })?;
        }

        stmt.finalize()?;

        Ok(())
    }

    pub fn get_all_data(&mut self) -> Result<Vec<Data>> {
        const QUERY: &str = "
SELECT hash1, hash2, tool_attrs, def_path, function FROM hir
";
        let mut stmt = self.conn.prepare(QUERY)?;
        stmt.query_and_then([], |row| {
            eyre::Ok(Data {
                hash: PrimaryKey {
                    hash1: row.get::<_, i64>(0)?.cast_unsigned(),
                    hash2: row.get::<_, i64>(1)?.cast_unsigned(),
                },
                func: Func {
                    tool_attrs: serde_json::from_str(row.get_ref(2)?.as_str()?)?,
                    def_path: row.get(3)?,
                    function: row.get(4)?,
                },
            })
        })?
        .collect()
    }
}

#[test]
fn test_db() -> Result<()> {
    safety_tool::logger::init();
    let mut db = Database::new("a.sqlite3")?;
    db.save_data([Data {
        hash: PrimaryKey { hash1: 1, hash2: 2 },
        func: Func {
            tool_attrs: vec!["Safety".to_owned()],
            def_path: "a::b".to_owned(),
            function: "fn f() {}".to_owned(),
        },
    }])?;
    dbg!(db.get_all_data()?);
    Ok(())
}
