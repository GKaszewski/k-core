use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct FastEmbedAdapter {
    model: Arc<Mutex<TextEmbedding>>,
}

impl FastEmbedAdapter {
    pub fn new() -> anyhow::Result<Self> {
        let mut options = InitOptions::default();
        options.model_name = EmbeddingModel::AllMiniLML6V2;
        options.show_download_progress = false;

        let model = TextEmbedding::try_new(options)
            .map_err(|e| anyhow::anyhow!("Failed to init fastembed: {}", e))?;

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
        })
    }

    pub fn generate_embedding(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let model = self.model.clone();
        let text = text.to_string();

        // FastEmbed is blocking, so we run it in a blocking task if we are in an async context,
        // but since this method signature doesn't force async, we wrap the internal logic.
        // For strictly async usage in k-core:
        let embeddings = std::thread::scope(|s| {
            s.spawn(|| {
                let mut model = model
                    .lock()
                    .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
                model
                    .embed(vec![text], None)
                    .map_err(|e| anyhow::anyhow!("Embed error: {}", e))
            })
            .join()
            .map_err(|_| anyhow::anyhow!("Thread join error"))?
        })?;

        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No embedding generated"))
    }

    /// Async wrapper for use in async contexts (like Axum handlers)
    pub async fn generate_embedding_async(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let model = self.model.clone();
        let text = text.to_string();

        let embeddings = tokio::task::spawn_blocking(move || {
            let mut model = model
                .lock()
                .map_err(|e| anyhow::anyhow!("Lock error: {}", e))?;
            model
                .embed(vec![text], None)
                .map_err(|e| anyhow::anyhow!("Embed error: {}", e))
        })
        .await??;

        embeddings
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No embedding generated"))
    }
}
