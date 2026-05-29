use kern_engine::{KernIndex, SearchEngine};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let index = KernIndex::new("./test_index")?;
    index.index_docs("./docs")?;

    let engine = SearchEngine::new(index);
    let results = engine.query("git push rejected")?;

    println!("Results:");
    for r in results {
        println!("  - {}", r);
    }

    Ok(())
}
