use mongodb::bson::{oid::ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: ObjectId,
    pub user_id: ObjectId,
    pub name: String,           // 账户名称
    pub account_type: String,  // 账户类型（如银行卡、现金、支付宝等）
    pub balance: f64,          // 当前余额
    pub currency: String,      // 币种
    pub remark: Option<String>,
}
