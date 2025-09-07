use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use crate::db::MongoDB;
use crate::auth::AuthUser;
use mongodb::bson::oid::ObjectId;

// 删除订单 handler
pub async fn delete_order_handler(
    State(db): State<Arc<MongoDB>>,
    AuthUser(user_id): AuthUser,
    Path(order_id): Path<String>,
) -> Json<serde_json::Value> {
    let obj_id = match ObjectId::parse_str(&order_id) {
        Ok(oid) => oid,
        Err(_) => return Json(serde_json::json!({"success": false, "msg": "无效订单ID"})),
    };
    match db.delete_order(user_id, obj_id).await {
        Ok(true) => Json(serde_json::json!({"success": true})),
        Ok(false) => Json(serde_json::json!({"success": false, "msg": "未找到订单"})),
        Err(e) => Json(serde_json::json!({"success": false, "msg": format!("数据库错误: {:?}", e)})),
    }
}
