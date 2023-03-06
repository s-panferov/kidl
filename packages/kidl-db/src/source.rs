use kidl_syn::{lexer::tokenize, source::RopeSource};
use rowan::GreenNode;

use crate::ir::SchemaFile;

#[salsa::tracked]
pub fn parse(db: &dyn crate::Db, source: SchemaFile) -> GreenNode {
    let text = source.text(db);
    let cache = db.cache();
    let mut cache = cache.node();
    kidl_syn::parser::parse(tokenize(RopeSource::new(text.slice(..))), &mut cache)
}
