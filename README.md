# 记账系统

## 项目简介

本项目为个人的财务记账系统，后端基于 Axum+ MongoDB，前端基于 React + Ant Design + Vite，支持多账户、分类、收入/支出、、图表统计等功能。

## 目录结构

```
├── src/                # Rust 后端源码
├── finance-frontend/   # 前端 React 项目
├── Cargo.toml          # Rust 配置
├── package.json        # 前端依赖
```

## 快速启动

### 1. 启动 MongoDB

请确保本地已安装并启动 MongoDB，默认连接：`mongodb://localhost:27017`，数据库名：`finance`

### 2. 启动后端（Rust）

```powershell
cd todo_list
cargo run
```

后端默认监听 `http://0.0.0.0:3000`，API 路径前缀为 `/api`。

### 3. 启动前端（React）

```powershell
cd finance-frontend
npm install
npm run dev
```

前端默认监听 `http://localhost:5173`。

## 主要功能

- 账户、资产、预算、分类管理
- 订单（收支）录入、分页、筛选、删除
- 图表统计（柱状/折线/饼图，支出/收入分块，分类分组）
- Token 登录鉴权（sessionStorage）

## 常见问题

- 端口冲突：如 3000/5173 被占用，请修改 `main.rs` 或 Vite 配置
- MongoDB 未启动：请先启动数据库
- Token 失效：重新登录

如有问题欢迎反馈！
