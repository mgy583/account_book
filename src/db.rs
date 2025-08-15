// src/db.rs
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use futures::stream::StreamExt;

#[derive(Error, Debug)]
pub enum DBError {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    
    #[error("Invalid ID: {0}")]
    InvalidId(String),
    
    #[error("Document not found")]
    NotFound,
}

pub type DBResult<T> = Result<T, DBError>;

#[derive(Clone)]
pub struct MongoDB {
    client: Client,
    db_name: String,
}

impl MongoDB {
    pub async fn new(uri: &str, db_name: &str) -> DBResult<Self> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self {
            client,
            db_name: db_name.to_string(),
        })
    }
    
    fn collection<T: Send + Sync>(&self, name: &str) -> Collection<T> {
        self.client.database(&self.db_name).collection(name)
    }
    
    // Todo 相关操作
    pub async fn get_todos(&self) -> DBResult<Vec<Todo>> {
        let collection = self.collection::<Todo>("todos");
        let options = FindOptions::builder()
            .sort(doc! { "id": 1 })
            .build();
        
    let mut cursor = collection.find(doc! {}).await.map_err(DBError::MongoError)?;
        let mut todos = Vec::new();
        
        while let Some(todo) = cursor.next().await {
            let todo = todo.map_err(DBError::MongoError)?;
            todos.push(todo);
        }
        
        Ok(todos)
    }
    
    pub async fn create_todo(&self, text: String) -> DBResult<Todo> {
        let collection = self.collection::<Todo>("todos");
        
        // 获取下一个ID
    let count = collection.count_documents(doc! {}).await.map_err(DBError::MongoError)?;
        let id = count as u32 + 1;
        
        let todo = Todo {
            id: Some(ObjectId::new()),
            text,
            done: false,
        };
        
    collection.insert_one(&todo).await.map_err(DBError::MongoError)?;
        Ok(todo)
    }
    
    pub async fn delete_todo(&self, id: &ObjectId) -> DBResult<()> {
        let collection = self.collection::<Todo>("todos");
        let filter = doc! { "_id": id };
        
    let result = collection.delete_one(filter).await.map_err(DBError::MongoError)?;
        if result.deleted_count == 0 {
            return Err(DBError::NotFound);
        }
        
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub text: String,
    pub done: bool,
}