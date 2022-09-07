#![warn(missing_docs)]
#![deny(unsafe_code)]
//! a bbs application api server
///命令行相关的模块
pub mod cli;
///公用模块
pub mod common;
///数据库操作层
pub mod data;
///HTTP 服务
pub mod http;
///数据迁移
pub mod migrations;
///模型定义
pub mod models;
///路由
pub mod routes;
///异步运行时
pub mod rt;
///业务层
pub mod services;
