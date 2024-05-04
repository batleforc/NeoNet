use async_trait::async_trait;

use super::{
    repo_error::{
        RepoCreateError, RepoDeleteError, RepoFindAllError, RepoSelectError, RepoUpdateError,
    },
    Entity, PersistenceConfig, SearchEntity,
};

#[async_trait]
pub trait Repository<T, A, P>
where
    T: Entity,
    A: SearchEntity + ?Sized,
    P: PersistenceConfig + ?Sized,
{
    /// A function responsible for the creation of the Repository
    fn new(config: &P) -> Result<Self, String>
    where
        Self: Sized;

    async fn init(&self) -> Result<(), String>;

    /// Insert the received entity in the persistence system
    async fn create(&self, entity: T) -> Result<T, RepoCreateError>;

    /// Find and return one single record from the persistence system
    async fn find_one(&self, search: A) -> Result<T, RepoSelectError>;

    /// Find and return all records corresponding to the search criteria from the persistence system
    async fn find_all(&self, search: A) -> Result<Vec<T>, RepoFindAllError>;

    /// Update one single record already present in the persistence system
    async fn update(&self, entity: T) -> Result<T, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, id: String) -> Result<(), RepoDeleteError>;

    /// Delete all records corresponding to the search criteria from the persistence system
    async fn delete_many(&self, search: A) -> Result<u64, RepoDeleteError>;
}