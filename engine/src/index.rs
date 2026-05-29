use anyhow::Result;
use tantivy::{
    schema::{Schema, TEXT, STORED},
    Index, IndexWriter, TantivyDocument,
};
use tracing::info;
use walkdir::WalkDir;

use crate::parser::parse_markdown;

pub struct KernIndex {
    pub index: Index,
    pub schema: Schema,
}

impl KernIndex {
    pub fn new(index_path: &str) -> Result<Self> {
        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("body", TEXT | STORED);
        schema_builder.add_text_field("path", STORED);
        let schema = schema_builder.build();

        std::fs::create_dir_all(index_path)?;

        let index = if Index::exists(&tantivy::directory::MmapDirectory::open(index_path)?)? {
            info!("opening existing index at {}", index_path);
            Index::open_in_dir(index_path)?
        } else {
            info!("creating new index at {}", index_path);
            Index::create_in_dir(index_path, schema.clone())?
        };

        Ok(Self { index, schema })
    }

    pub fn writer(&self) -> Result<IndexWriter> {
        Ok(self.index.writer(50_000_000)?)
    }

    pub fn index_docs(&self, docs_path: &str) -> Result<()> {
        let mut writer = self.writer()?;

        let title_field = self.schema.get_field("title")?;
        let body_field = self.schema.get_field("body")?;
        let path_field = self.schema.get_field("path")?;

        writer.delete_all_documents()?;

        for entry in WalkDir::new(docs_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "md")
                    .unwrap_or(false)
            })
        {
            let path = entry.path().to_str().unwrap_or("").to_string();
            match parse_markdown(&path) {
                Ok(doc) => {
                    let mut tantivy_doc = TantivyDocument::default();
                    tantivy_doc.add_text(title_field, &doc.title);
                    tantivy_doc.add_text(body_field, &doc.body);
                    tantivy_doc.add_text(path_field, &doc.path);
                    writer.add_document(tantivy_doc)?;
                    info!("indexed: {}", path);
                }
                Err(e) => {
                    tracing::warn!("failed to parse {}: {}", path, e);
                }
            }
        }

        writer.commit()?;
        info!("index committed");

        Ok(())
    }
}