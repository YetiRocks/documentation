//! Documentation Build Tool
//!
//! Builds the mdBook documentation programmatically.
//! This can be run with: cargo run --bin build-docs

use anyhow::Result;
use mdbook::MDBook;

fn main() -> Result<()> {
    // Get the directory containing this Cargo.toml
    let app_dir = std::env::current_dir()?;
    let source_dir = app_dir.join("source");
    let web_dir = app_dir.join("web");

    println!("Building Yeti documentation from {:?}", source_dir);
    println!("Output directory: {:?}", web_dir);

    // Load and build the mdBook
    let md = MDBook::load(&source_dir)?;
    md.build()?;

    println!("✓ Documentation built successfully at {:?}", web_dir);

    Ok(())
}
