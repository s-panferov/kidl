use cache::Cache;
pub use db::Database;

mod cache;
mod db;

pub mod ir;
pub mod source;

// ANCHOR: jar_struct
#[salsa::jar(db = Db)]
pub struct Jar(crate::ir::SchemaFile, crate::source::parse);

pub trait Db: salsa::DbWithJar<Jar> {
    fn cache(&self) -> Cache;
}
