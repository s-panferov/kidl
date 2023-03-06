use std::path::PathBuf;

#[salsa::input]
pub struct SchemaFile {
    #[return_ref]
    pub path: PathBuf,
    pub text: ropey::Rope,
}
