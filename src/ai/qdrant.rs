use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, PointStruct, SearchPointsBuilder, UpsertPointsBuilder,
    Value, VectorParamsBuilder,
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct QdrantAdapter {
    client: Arc<Qdrant>,
    collection_name: String,
}

impl QdrantAdapter {
    pub fn new(url: &str, collection_name: &str) -> anyhow::Result<Self> {
        let client = Qdrant::from_url(url).build()?;
        Ok(Self {
            client: Arc::new(client),
            collection_name: collection_name.to_string(),
        })
    }

    pub async fn create_collection_if_not_exists(&self, vector_size: u64) -> anyhow::Result<()> {
        if !self.client.collection_exists(&self.collection_name).await? {
            self.client
                .create_collection(
                    CreateCollectionBuilder::new(self.collection_name.clone())
                        .vectors_config(VectorParamsBuilder::new(vector_size, Distance::Cosine)),
                )
                .await?;
        }
        Ok(())
    }

    pub async fn upsert(
        &self,
        id: Uuid,
        vector: Vec<f32>,
        payload: HashMap<String, Value>,
    ) -> anyhow::Result<()> {
        let point = PointStruct::new(id.to_string(), vector, payload);
        let upsert_points = UpsertPointsBuilder::new(self.collection_name.clone(), vec![point]);

        self.client.upsert_points(upsert_points).await?;
        Ok(())
    }

    pub async fn search(&self, vector: Vec<f32>, limit: u64) -> anyhow::Result<Vec<(Uuid, f32)>> {
        let search_points = SearchPointsBuilder::new(self.collection_name.clone(), vector, limit)
            .with_payload(true);

        let search_result = self.client.search_points(search_points).await?;

        let results = search_result
            .result
            .into_iter()
            .filter_map(|point| {
                let id = point.id?;
                let uuid_str = match id.point_id_options? {
                    qdrant_client::qdrant::point_id::PointIdOptions::Uuid(u) => u,
                    _ => return None,
                };
                let uuid = Uuid::parse_str(&uuid_str).ok()?;
                Some((uuid, point.score))
            })
            .collect();

        Ok(results)
    }
}
