use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use crate::{cache::Cache, ir::SchemaFile, Db};

#[derive(Default)]
#[salsa::db(crate::Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
    cache: Cache,
    schema_files: HashMap<PathBuf, SchemaFile>,
}

impl Db for Database {
    fn cache(&self) -> Cache {
        self.cache.clone()
    }
}

impl Database {
    pub fn schema_file(&self, path: &Path) -> Option<SchemaFile> {
        self.schema_files.get(path).cloned()
    }

    pub fn push_file(&mut self, path: PathBuf, source: String) {
        let schema_file = SchemaFile::new(self, path.clone(), ropey::Rope::from(source));
        self.schema_files.insert(path, schema_file);
    }

    pub fn push_files(&mut self, files: Vec<(PathBuf, String)>) {
        for file in files {
            self.push_file(file.0, file.1)
        }
    }
}

impl salsa::Database for Database {}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Database {
            storage: self.storage.snapshot(),
            schema_files: self.schema_files.clone(),
            cache: self.cache.clone(),
        })
    }
}
