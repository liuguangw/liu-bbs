use super::MigrationError;
use async_trait::async_trait;

///数据迁移trait定义
#[async_trait]
pub trait Migration: Send + Sync {
    ///迁移的唯一名称
    fn name(&self) -> &str;
    ///执行迁移, 函数签名如下
    ///
    ///```ignore
    ///async fn up(&self) -> Result<(), MigrationError>
    ///```
    async fn up(&self) -> Result<(), MigrationError>;
    ///回滚迁移, 函数签名如下
    ///
    ///```ignore
    ///async fn down(&self) -> Result<(), MigrationError>
    ///```
    async fn down(&self) -> Result<(), MigrationError>;
}
