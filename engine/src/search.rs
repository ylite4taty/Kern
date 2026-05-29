use anyhow::Result;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::Value;
use tantivy::TantivyDocument;
use tracing::info;

use crate::KernIndex;

pub struct SearchEngine {
    kern_index: KernIndex,

}

impl SearchEngine {
    pub fn new(kern_index: KernIndex) -> Self {
        Self { kern_index }

    }

    pub fn query(&self, input: &str) -> Result<Vec<String>> {
        let reader = self.kern_index.index.reader()?;
        let searcher = reader.searcher();

        let title = self.kern_index.schema.get_field("title")?;
        let body = self.kern_index.schema.get_field("body")?;

        let query_parser = QueryParser::for_index(&self.kern_index.index, vec![title, body]);
        let query = query_parser.parse_query(input)?;

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let doc: TantivyDocument = searcher.doc(doc_address)?;
            let title_val = doc
                .get_first(title)
                .and_then(|v| v.as_str())
                .unwrap_or("untitled")
                .to_string();
            results.push(title_val);
        }

        info!("query '{}' returned {} results", input, results.len());
        Ok(results)
    }

}
