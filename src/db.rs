use crate::models::user::User;
use crate::models::account::Account;
use crate::models::category::Category;
use crate::models::asset::Asset;
use crate::models::transaction::Order;
use crate::models::budget::Budget;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use futures::stream::TryStreamExt;
use mongodb::bson::{oid::ObjectId, DateTime};

type DBResult<T> = Result<T, mongodb::error::Error>;
pub struct MongoDB {
    client: Client,
    pub db: mongodb::Database,
    pub accounts: Collection<Account>,
    pub categories: Collection<Category>,
    pub assets: Collection<Asset>,
    pub orders: Collection<Order>,
    pub budgets: Collection<Budget>,
}

impl MongoDB {
    pub async fn new(uri: &str, db_name: &str) -> DBResult<Self> {
        let client = Client::with_uri_str(uri).await?;
        let db = client.database(db_name);
        Ok(Self {
            client,
            db: db.clone(),
            accounts: db.collection::<Account>("accounts"),
            categories: db.collection::<Category>("categories"),
            assets: db.collection::<Asset>("assets"),
            orders: db.collection::<Order>("orders"),
            budgets: db.collection::<Budget>("budgets"),
        })
    }

    pub fn users_collection(&self) -> mongodb::Collection<User> {
        self.db.collection::<User>("users")
    }
    // 账户相关
    pub async fn create_account(&self, user_id: ObjectId, name: String, account_type: String, balance: f64, currency: String, remark: Option<String>) -> DBResult<Account> {
        let account = Account {
            id: ObjectId::new(),
            user_id,
            name,
            account_type,
            balance,
            currency,
            remark,
        };
        self.accounts.insert_one(&account).await?;
        Ok(account)
    }
    pub async fn get_accounts_by_user(&self, user_id: ObjectId) -> DBResult<Vec<Account>> {
        let mut cursor = self.accounts.find(doc! {"user_id": &user_id}).await?;
        let mut accounts = Vec::new();
        while let Some(account) = cursor.try_next().await? {
            accounts.push(account);
        }
        Ok(accounts)
    }

    // 分类相关
    pub async fn create_category(&self, user_id: ObjectId, name: String, parent_id: Option<ObjectId>, category_type: String) -> DBResult<Category> {
        let category = Category {
            id: ObjectId::new(),
            user_id,
            name,
            parent_id,
            category_type,
        };
        self.categories.insert_one(&category).await?;
        Ok(category)
    }

    pub async fn get_categories_by_user(&self, user_id: ObjectId) -> DBResult<Vec<Category>> {
        let mut cursor = self.categories.find(doc! {"user_id": &user_id}).await?;
        let mut categories = Vec::new();
        while let Some(category) = cursor.try_next().await? {
            categories.push(category);
        }
        Ok(categories)
    }

    // 资产相关
    pub async fn create_asset(&self, user_id: ObjectId, name: String, asset_type: String, value: f64, currency: String, account_id: ObjectId, remark: Option<String>) -> DBResult<Asset> {
        let asset = Asset {
            id: ObjectId::new(),
            user_id,
            name,
            asset_type,
            value,
            currency,
            account_id,
            remark,
        };
        self.assets.insert_one(&asset).await?;
        Ok(asset)
    }

    pub async fn get_assets_by_user(&self, user_id: ObjectId) -> DBResult<Vec<Asset>> {
        let mut cursor = self.assets.find(doc! {"user_id": &user_id}).await?;
        let mut assets = Vec::new();
        while let Some(asset) = cursor.try_next().await? {
            assets.push(asset);
        }
        Ok(assets)
    }

    // 订单相关
    pub async fn create_order(&self, user_id: ObjectId, name: String, order_type: String, amount: f64, currency: String, date: DateTime, remark: Option<String>) -> DBResult<Order> {
        let order = Order {
            id: ObjectId::new(),
            user_id,
            name,
            order_type,
            amount,
            currency,
            date,
            remark,
        };
        self.orders.insert_one(&order).await?;
        Ok(order)
    }

    pub async fn get_orders_by_user(&self, user_id: ObjectId) -> DBResult<Vec<Order>> {
        let mut cursor = self.orders.find(doc! {"user_id": &user_id}).await?;
        let mut orders = Vec::new();
        while let Some(order) = cursor.try_next().await? {
            orders.push(order);
        }
        Ok(orders)
    }

    pub async fn delete_order(&self, user_id: ObjectId, order_id: ObjectId) -> DBResult<bool> {
    let res = self.orders.delete_one(doc! {"user_id": &user_id, "_id": order_id}).await?;
        Ok(res.deleted_count > 0)
    }

    // 预算相关
    pub async fn create_budget(&self, user_id: ObjectId, category_id: ObjectId, amount: f64, period: String, start_date: DateTime, end_date: DateTime) -> DBResult<Budget> {
        let budget = Budget {
            id: ObjectId::new(),
            user_id,
            category_id,
            amount,
            period,
            start_date,
            end_date,
        };
        self.budgets.insert_one(&budget).await?;
        Ok(budget)
    }

    pub async fn get_budgets_by_user(&self, user_id: ObjectId) -> DBResult<Vec<Budget>> {
        let mut cursor = self.budgets.find(doc! {"user_id": &user_id}).await?;
        let mut budgets = Vec::new();
        while let Some(budget) = cursor.try_next().await? {
            budgets.push(budget);
        }
        Ok(budgets)
    }
}