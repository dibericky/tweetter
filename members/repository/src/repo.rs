use crate::postgres::get_connection;
use diesel::PgConnection;

pub struct Repository {
    conn: PgConnection,
}

impl Repository {
    pub fn new() -> Self {
        Self {
            conn: get_connection(),
        }
    }

    pub fn get_connection(&mut self) -> &mut PgConnection {
        &mut self.conn
    }
}

impl Default for Repository {
    fn default() -> Self {
        Self::new()
    }
}