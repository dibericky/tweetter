use std::fmt::{self, Display};

use serde::{Serialize,Deserialize};
use jfs::Store;

pub struct Repository {
    tweets: Store
}

pub enum Table {
    TWEETS
}

pub fn new_store (name: Table) -> Store {
    let mut cfg = jfs::Config::default();
    cfg.single = true; // false is default
    Store::new_with_cfg(name.to_string(),cfg).unwrap()
}

impl Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Table::TWEETS => write!(f, "tweets"),
        }
    }
}

impl Repository {
    pub fn new() -> Self {
        Self {
            tweets: new_store(Table::TWEETS)
        }
    }

    fn get_store(&self, table: Table) -> &Store {
        match table {
            Table::TWEETS => &self.tweets
        }
    }

    pub fn add<T>(&self, table: Table, doc: &T) -> String where T: Serialize + for<'a> Deserialize<'a> {
        self.get_store(table).save(doc).unwrap()
    }

    pub fn get<T>(&self, table: Table, id: &str) -> T where T: Serialize + for<'a> Deserialize<'a> {
        self.get_store(table).get::<T>(id).unwrap()
    }
}