use mongodb::bson::doc;

use crate::database::{repo_error::RepoInitError, PersistenceConfig};

#[derive(Debug, Clone)]
pub struct MongoDbConfig {
    pub uri: String,
    pub db_name: String,
    pub client: Option<mongodb::Client>,
}

impl PersistenceConfig for MongoDbConfig {}

impl MongoDbConfig {
    pub fn new(uri: String, db_name: String) -> Self {
        Self {
            uri,
            db_name,
            client: None,
        }
    }
    pub async fn init_db(&mut self) -> Result<mongodb::Client, RepoInitError> {
        let client = mongodb::Client::with_uri_str(&self.uri)
            .await
            .map_err(|err| RepoInitError::Unknown(err.to_string()))?;
        self.client = Some(client.clone());
        Ok(client.clone())
    }
    pub async fn get_client(&mut self) -> Result<mongodb::Client, RepoInitError> {
        if let Some(client) = &self.client {
            return Ok(client.clone());
        } else {
            return self.init_db().await;
        }
    }
    pub async fn get_database(&mut self) -> Result<mongodb::Database, RepoInitError> {
        let client = self.get_client().await?;
        Ok(client.database(&self.db_name))
    }
    pub async fn health_check(&mut self) -> Result<(), RepoInitError> {
        let database = self.get_database().await?;
        database
            .run_command(doc! { "ping": 1 }, None)
            .await
            .map_err(|err| RepoInitError::Unknown(err.to_string()))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mongodb_config() {
        let uri = "mongodb://localhost:27017".to_string();
        let db_name = "test_db".to_string();
        let mut config = MongoDbConfig::new(uri.clone(), db_name.clone());
        assert_eq!(config.uri, uri);
        assert_eq!(config.db_name, db_name);
        assert!(config.client.is_none());
        let _ = config.init_db().await;
        assert!(config.client.is_some());
        let database = config.get_database().await.unwrap();
        assert_eq!(database.name(), db_name);
        config.health_check().await.unwrap();
    }
}
