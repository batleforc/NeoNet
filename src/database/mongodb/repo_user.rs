use super::{config::MongoDbConfig, user::UserMongo};
use crate::database::mongodb::repo_user::bson::doc;
use crate::database::user::SearchUser;
use crate::{database::repo::Repository, model::user::User};
use async_trait::async_trait;
use futures_util::StreamExt;
use mongodb::bson;

#[derive(Debug, Clone)]
pub struct UserMongoRepo {
    pub db: mongodb::Database,
}

#[async_trait]
impl Repository<User, SearchUser, MongoDbConfig> for UserMongoRepo {
    fn new(config: &MongoDbConfig) -> Result<Self, String> {
        match &config.client {
            Some(client) => Ok(Self {
                db: client.database(&config.db_name),
            }),
            None => Err("Client not initialized".to_string()),
        }
    }
    #[tracing::instrument(level = "debug")]
    async fn init(&self) -> Result<(), String> {
        Ok(())
    }

    #[tracing::instrument(level = "debug")]
    async fn create(
        &self,
        entity: User,
    ) -> Result<User, crate::database::repo_error::RepoCreateError> {
        let collection = UserMongo::get_collection(self.db.clone());
        let user_mongo = UserMongo::from(entity.clone());
        match collection.insert_one(user_mongo, None).await {
            Ok(test) => {
                let mut entity = entity;
                entity.id = test.inserted_id.as_object_id().unwrap().to_hex();
                Ok(entity)
            }
            Err(e) => Err(crate::database::repo_error::RepoCreateError::Unknown(
                e.to_string(),
            )),
        }
    }

    #[tracing::instrument(level = "debug")]
    async fn find_one(
        &self,
        search: SearchUser,
    ) -> Result<User, crate::database::repo_error::RepoSelectError> {
        let collection = UserMongo::get_collection(self.db.clone());
        let search_doc = search.turn_into_search();
        println!("{:?}", search_doc);
        match collection.find_one(search_doc, None).await {
            Ok(Some(doc)) => match doc.try_into() {
                Ok(user) => Ok(user),
                Err(_) => Err(crate::database::repo_error::RepoSelectError::Unknown(
                    "Error converting document to User".to_string(),
                )),
            },
            Ok(None) => Err(crate::database::repo_error::RepoSelectError::NoRowFound),
            Err(e) => Err(crate::database::repo_error::RepoSelectError::Unknown(
                e.to_string(),
            )),
        }
    }

    #[tracing::instrument(level = "debug")]
    async fn find_all(
        &self,
        search: SearchUser,
    ) -> Result<Vec<User>, crate::database::repo_error::RepoFindAllError> {
        let collection = UserMongo::get_collection(self.db.clone());
        let search = search.turn_into_search();
        let mut cursor = match collection.find(search, None).await {
            Ok(cursor) => cursor,
            Err(e) => {
                return Err(crate::database::repo_error::RepoFindAllError::Unknown(
                    e.to_string(),
                ))
            }
        };
        let mut users = Vec::new();
        while let Some(doc) = cursor.next().await {
            match doc {
                Ok(doc) => match doc.try_into() {
                    Ok(user) => users.push(user),
                    Err(_) => {
                        return Err(crate::database::repo_error::RepoFindAllError::Unknown(
                            "Error converting document to User".to_string(),
                        ))
                    }
                },
                Err(e) => {
                    return Err(crate::database::repo_error::RepoFindAllError::Unknown(
                        e.to_string(),
                    ))
                }
            }
        }
        Ok(users)
    }

    #[tracing::instrument(level = "debug")]
    async fn update(
        &self,
        entity: User,
    ) -> Result<User, crate::database::repo_error::RepoUpdateError> {
        let collection = UserMongo::get_collection(self.db.clone());
        let user_mongo = UserMongo::from(entity.clone());
        let doc = bson::to_document(&user_mongo).unwrap();
        collection
            .update_one(doc! { "_id": &user_mongo._id }, doc! { "$set": doc }, None)
            .await
            .unwrap();
        Ok(entity)
    }

    #[tracing::instrument(level = "debug")]
    async fn delete(&self, id: String) -> Result<(), crate::database::repo_error::RepoDeleteError> {
        let collection = UserMongo::get_collection(self.db.clone());
        collection
            .delete_one(doc! { "_id": &id }, None)
            .await
            .unwrap();
        Ok(())
    }

    #[tracing::instrument(level = "debug")]
    async fn delete_many(
        &self,
        search: SearchUser,
    ) -> Result<u64, crate::database::repo_error::RepoDeleteError> {
        let collection = UserMongo::get_collection(self.db.clone());
        let search = search.turn_into_search();
        let result = collection
            .delete_many(search.or_else(|| Some(doc! {})).unwrap(), None)
            .await
            .unwrap();
        Ok(result.deleted_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::role::Role;
    use chrono::Utc;
    use mongodb::bson::oid::ObjectId;
    use mongodb::Client;
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_create() {
        let client = Client::with_uri_str("mongodb://root:root@localhost:27017")
            .await
            .unwrap();
        let db = client.database("test");
        let repo = UserMongoRepo { db };
        repo.delete_many(SearchUser::default()).await.unwrap();
        let user = User::new(
            ObjectId::new().to_string(),
            Role::User,
            true,
            "test".to_string(),
            "test".to_string(),
            None,
            None,
            None,
            None,
            Utc::now(),
            Utc::now(),
            Vec::new(),
        );
        let result = repo.create(user.clone()).await.unwrap();
        assert!(result.id == user.id);
        repo.delete_many(SearchUser::default()).await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_find_one() {
        dotenvy::from_path(".env").unwrap();
        let client = Client::with_uri_str("mongodb://root:root@localhost:27017")
            .await
            .unwrap();
        let db = client.database("test");
        let repo = UserMongoRepo { db };
        repo.delete_many(SearchUser::default()).await.unwrap();
        let user = User::new(
            ObjectId::new().to_string(),
            Role::User,
            true,
            "test".to_string(),
            "test".to_string(),
            None,
            None,
            None,
            None,
            Utc::now(),
            Utc::now(),
            Vec::new(),
        );
        repo.create(user.clone()).await.unwrap();
        let search = SearchUser {
            username: Some("test".to_string()),
            ..Default::default()
        };
        let result = repo.find_one(search).await.unwrap();
        assert!(result.id == user.id);
        repo.delete_many(SearchUser::default()).await.unwrap();
    }

    #[tokio::test]
    #[serial]
    async fn test_find_all() {
        let client = Client::with_uri_str("mongodb://root:root@localhost:27017")
            .await
            .unwrap();
        let db = client.database("test");
        let repo = UserMongoRepo { db };
        repo.delete_many(SearchUser::default()).await.unwrap();
        let user = User::new(
            ObjectId::new().to_string(),
            Role::User,
            true,
            "test".to_string(),
            "test".to_string(),
            None,
            None,
            None,
            None,
            Utc::now(),
            Utc::now(),
            Vec::new(),
        );
        repo.create(user.clone()).await.unwrap();
        let search = SearchUser {
            username: Some("/te.*/".to_string()),
            ..Default::default()
        };

        let result = repo.find_all(search).await.unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].id == user.id);
        repo.delete_many(SearchUser::default()).await.unwrap();
    }
}
