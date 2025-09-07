use mongodb::bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: ObjectId,
    pub user_id: ObjectId,      // 用户ID
    pub name: String,           // 资产名称（如股票、基金、房产等）
    pub asset_type: String,     // 资产类型
    pub value: f64,             // 当前市值
    pub currency: String,       // 币种
    pub account_id: ObjectId,   // 关联账户
    pub remark: Option<String>,
}
