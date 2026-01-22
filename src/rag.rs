use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use anyhow::Result;

pub struct Brain {
    model: TextEmbedding,
}

impl Brain {
    // Initialize the AI Brain
    pub fn new() -> Result<Self> {
        println!("🧠 Loading AI Model (this might take a moment)...");
        
        // FIX 1: Use default() and assign fields manually
        // This bypasses the "non-exhaustive" error
        let mut options = InitOptions::default();
        options.model_name = EmbeddingModel::AllMiniLML6V2;
        options.show_download_progress = true;

        let model = TextEmbedding::try_new(options)?;
        Ok(Self { model })
    }

    // FIX 2: Changed &self to &mut self
    // The model needs mutable access to run calculations
    pub fn embed(&mut self, text: &str) -> Result<Vec<f32>> {
        let documents = vec![text.to_string()];
        let embeddings = self.model.embed(documents, None)?;
        // Return the first (and only) embedding
        Ok(embeddings[0].clone())
    }
}