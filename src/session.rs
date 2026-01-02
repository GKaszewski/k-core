// Only compile this module if we have the store dependency AND sqlx enabled
#[cfg(all(feature = "sessions-db", feature = "db-sqlx"))]
pub mod store {
    use async_trait::async_trait;
    use tower_sessions::{
        SessionStore,
        session::{Id, Record},
        session_store,
    };

    // We only import what is actually enabled by features
    #[cfg(feature = "postgres")]
    use tower_sessions_sqlx_store::PostgresStore;
    #[cfg(feature = "sqlite")]
    use tower_sessions_sqlx_store::SqliteStore;

    #[derive(Clone, Debug)]
    pub enum InfraSessionStore {
        #[cfg(feature = "sqlite")]
        Sqlite(SqliteStore),
        #[cfg(feature = "postgres")]
        Postgres(PostgresStore),
        // Fallback variant to allow compilation if sessions-db is enabled but no generic DB feature is selected
        // (Though in practice you should ensure your config validates this)
        #[allow(dead_code)]
        Unused,
    }

    #[async_trait]
    impl SessionStore for InfraSessionStore {
        async fn save(&self, session_record: &Record) -> session_store::Result<()> {
            match self {
                #[cfg(feature = "sqlite")]
                Self::Sqlite(store) => store.save(session_record).await,
                #[cfg(feature = "postgres")]
                Self::Postgres(store) => store.save(session_record).await,
                _ => Err(session_store::Error::Backend(
                    "No database feature enabled".to_string(),
                )),
            }
        }

        async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
            match self {
                #[cfg(feature = "sqlite")]
                Self::Sqlite(store) => store.load(session_id).await,
                #[cfg(feature = "postgres")]
                Self::Postgres(store) => store.load(session_id).await,
                _ => Err(session_store::Error::Backend(
                    "No database feature enabled".to_string(),
                )),
            }
        }

        async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
            match self {
                #[cfg(feature = "sqlite")]
                Self::Sqlite(store) => store.delete(session_id).await,
                #[cfg(feature = "postgres")]
                Self::Postgres(store) => store.delete(session_id).await,
                _ => Err(session_store::Error::Backend(
                    "No database feature enabled".to_string(),
                )),
            }
        }
    }

    impl InfraSessionStore {
        pub async fn migrate(&self) -> anyhow::Result<()> {
            match self {
                #[cfg(feature = "sqlite")]
                Self::Sqlite(store) => store.migrate().await.map_err(|e| anyhow::anyhow!(e)),
                #[cfg(feature = "postgres")]
                Self::Postgres(store) => store.migrate().await.map_err(|e| anyhow::anyhow!(e)),
                _ => Ok(()), // No migration needed for no-op
            }
        }
    }
}
