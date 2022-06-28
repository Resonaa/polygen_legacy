use rusqlite::{Connection, Params, Result, Statement};

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("data.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS user (
                      uid              INTEGER PRIMARY KEY,
                      username         TEXT,
                      password         TEXT,
                      exp              INTEGER
                      )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn execute(&self, sql: &str, params: impl Params) -> Result<usize> {
        self.conn.execute(sql, params)
    }

    pub fn prepare(&self, sql: &str) -> Result<Statement<'_>> {
        self.conn.prepare(sql)
    }
}
