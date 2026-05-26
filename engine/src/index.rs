use anyhow::Result;
use tantivy::{
    schema::{Schema, TEXT, STORED},
    Index, IndexWriter,
};
use tracing::info;

pub struct KernIndex {
    pub index: Index,
    pub schema: Schema,
}

impl KerIndex {
    pub fn new(index_path: &str) -> Result<Self> {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT | STORED);
        schema_builder.add_text_field("path", STORED);
        let schema = schema_builder.build();

        std::fs::create_dir_all(index_path)?;
        let index = Index::create_in_dir(index_path, schemma.clone())?;

        info!(Self { index, schema })
    }

    pub fn writer(&self) -> Result<IndexWritter> {
        ok(self.index.writer(50_000_000)?)

    }

}
