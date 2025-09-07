use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub category_id: ObjectId,  // 预算分类
    pub amount: f64,            // 预算金额
    pub period: String,         // 预算周期（月/年/自定义）
    pub start_date: DateTime,
    pub end_date: DateTime,
}
