use mongodb::bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub name: String,           // 分类名称（如餐饮、交通、工资等）
    pub parent_id: Option<ObjectId>, // 父级分类
    pub category_type: String,  // 类型（收入/支出/转账）
}
