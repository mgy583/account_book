use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub order_type: String,     // 类型（消费/收入/转账）
    pub amount: f64,           // 金额
    pub currency: String,      // 币种（人民币/美元/欧元等）
    pub date: DateTime,        // 日期
    pub remark: Option<String>,
}
