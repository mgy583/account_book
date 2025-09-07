use reqwest::Client;
use serde_json::json;

#[tokio::test]
async fn test_account_crud() {
    let client = Client::new();
    // 创建账户
    let resp = client.post("http://localhost:3000/accounts")
        .json(&json!({
            "name": "测试账户",
            "account_type": "银行卡",
            "balance": 1000.0,
            "currency": "CNY",
            "remark": "测试"
        }))
        .send().await.unwrap();
    assert!(resp.status().is_success());
    let account: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(account["name"], "测试账户");

    // 查询账户
    let resp = client.get("http://localhost:3000/accounts").send().await.unwrap();
    assert!(resp.status().is_success());
    let accounts: serde_json::Value = resp.json().await.unwrap();
    assert!(accounts.as_array().unwrap().iter().any(|a| a["name"] == "测试账户"));
}

#[tokio::test]
async fn test_category_crud() {
    let client = Client::new();
    // 创建分类
    let resp = client.post("http://localhost:3000/categories")
        .json(&json!({
            "name": "餐饮",
            "parent_id": null,
            "category_type": "支出"
        }))
        .send().await.unwrap();
    assert!(resp.status().is_success());
    let category: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(category["name"], "餐饮");

    // 查询分类
    let resp = client.get("http://localhost:3000/categories").send().await.unwrap();
    assert!(resp.status().is_success());
    let categories: serde_json::Value = resp.json().await.unwrap();
    assert!(categories.as_array().unwrap().iter().any(|c| c["name"] == "餐饮"));
}

#[tokio::test]
async fn test_asset_crud() {
    let client = Client::new();
    // 先查账户获取 id
    let resp = client.get("http://localhost:3000/accounts").send().await.unwrap();
    let accounts: serde_json::Value = resp.json().await.unwrap();
    let account_id = accounts[0]["id"].as_str().unwrap();
    // 创建资产
    let resp = client.post("http://localhost:3000/assets")
        .json(&json!({
            "name": "股票A",
            "asset_type": "股票",
            "value": 5000.0,
            "currency": "CNY",
            "account_id": account_id,
            "remark": "测试资产"
        }))
        .send().await.unwrap();
    assert!(resp.status().is_success());
    let asset: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(asset["name"], "股票A");

    // 查询资产
    let resp = client.get("http://localhost:3000/assets").send().await.unwrap();
    assert!(resp.status().is_success());
    let assets: serde_json::Value = resp.json().await.unwrap();
    assert!(assets.as_array().unwrap().iter().any(|a| a["name"] == "股票A"));
}

#[tokio::test]
async fn test_transaction_crud() {
    let client = Client::new();
    // 先查账户和分类获取 id
    let accounts: serde_json::Value = client.get("http://localhost:3000/accounts").send().await.unwrap().json().await.unwrap();
    let categories: serde_json::Value = client.get("http://localhost:3000/categories").send().await.unwrap().json().await.unwrap();
    let account_id = accounts[0]["id"].as_str().unwrap();
    let category_id = categories[0]["id"].as_str().unwrap();
    // 创建交易
    let now = chrono::Utc::now().to_rfc3339();
    let resp = client.post("http://localhost:3000/transactions")
        .json(&json!({
            "account_id": account_id,
            "category_id": category_id,
            "amount": 88.8,
            "currency": "CNY",
            "transaction_type": "支出",
            "time": now,
            "remark": "测试交易"
        }))
        .send().await.unwrap();
    assert!(resp.status().is_success());
    let tx: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(tx["amount"], 88.8);

    // 查询交易
    let resp = client.get("http://localhost:3000/transactions").send().await.unwrap();
    assert!(resp.status().is_success());
    let txs: serde_json::Value = resp.json().await.unwrap();
    assert!(txs.as_array().unwrap().iter().any(|t| t["amount"] == 88.8));
}

#[tokio::test]
async fn test_budget_crud() {
    let client = Client::new();
    // 先查分类获取 id
    let categories: serde_json::Value = client.get("http://localhost:3000/categories").send().await.unwrap().json().await.unwrap();
    let category_id = categories[0]["id"].as_str().unwrap();
    let now = chrono::Utc::now();
    let start = now.to_rfc3339();
    let end = (now + chrono::Duration::days(30)).to_rfc3339();
    // 创建预算
    let resp = client.post("http://localhost:3000/budgets")
        .json(&json!({
            "category_id": category_id,
            "amount": 1000.0,
            "period": "月",
            "start_date": start,
            "end_date": end
        }))
        .send().await.unwrap();
    assert!(resp.status().is_success());
    let budget: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(budget["amount"], 1000.0);

    // 查询预算
    let resp = client.get("http://localhost:3000/budgets").send().await.unwrap();
    assert!(resp.status().is_success());
    let budgets: serde_json::Value = resp.json().await.unwrap();
    assert!(budgets.as_array().unwrap().iter().any(|b| b["amount"] == 1000.0));
}
